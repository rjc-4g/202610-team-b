// Node.js標準機能のみで、実際のapp.jsの通信・タイマー・表示更新を検証する。
// DOMと時刻を代替する単体テスト。実ブラウザのレイアウト確認は別途行う。
const { test } = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");
const source = fs.readFileSync(path.join(__dirname, "../static/app.js"), "utf8");

// 任意の通信処理を受け取り、画面要素と手動で進められるタイマーを返す。
function createDashboard(fetch) {
  const elements = Object.fromEntries(
    ["cpu", "memory", "cpu-meter", "memory-meter", "status", "updated", "error"].map((id) => [
      `#${id}`,
      { textContent: "", hidden: true, dataset: {}, attributes: {}, setAttribute(name, value) { this.attributes[name] = value; } },
    ]),
  );
  const intervals = [];
  const timeouts = new Map();
  let timerId = 0;
  const context = vm.createContext({
    document: { querySelector: (selector) => elements[selector] },
    fetch,
    AbortController,
    setInterval: (callback, delay) => { intervals.push({ callback, delay }); },
    setTimeout: (callback, delay) => { timeouts.set(++timerId, { callback, delay }); return timerId; },
    clearTimeout: (id) => timeouts.delete(id),
  });
  vm.runInContext(source, context);
  return { elements, intervals, timeouts };
}

// Promiseの処理が完了するまで待つ。実時間の5秒待機は不要。
const settle = () => new Promise((resolve) => setImmediate(resolve));
const ok = (cpu = 42.5, memory = 61.2) => ({ ok: true, json: async () => ({ cpu_percent: cpu, memory_percent: memory }) });

test("初回取得と5秒間隔の更新で値・接続状態・日時を表示する", async () => {
  const calls = [];
  const ui = createDashboard(async (url, options) => { calls.push({ url, options }); return ok(calls.length === 1 ? 42.5 : 15, 61.2); });
  await settle();
  assert.equal(calls.length, 1);
  assert.equal(calls[0].url, "/api/metrics");
  assert.equal(calls[0].options.cache, "no-store");
  assert.equal(ui.elements["#cpu"].textContent, "42.5%");
  assert.equal(ui.elements["#memory"].textContent, "61.2%");
  assert.equal(ui.elements["#cpu-meter"].value, 42.5);
  assert.equal(ui.elements["#status"].textContent, "接続済み");
  assert.ok(ui.elements["#updated"].dateTime);
  assert.equal(ui.intervals.length, 1);
  assert.equal(ui.intervals[0].delay, 5000);
  await ui.intervals[0].callback();
  assert.equal(calls.length, 2);
  assert.equal(ui.elements["#cpu"].textContent, "15.0%");
  assert.equal(ui.timeouts.size, 0);
});

test("通信失敗時は前回の値と日時を保持し、次回成功するとエラーを解除する", async () => {
  let connected = true;
  const ui = createDashboard(async () => { if (!connected) throw new TypeError("Network error"); return ok(); });
  await settle();
  const previousTime = ui.elements["#updated"].dateTime;
  connected = false;
  await ui.intervals[0].callback();
  assert.equal(ui.elements["#cpu"].textContent, "42.5%");
  assert.equal(ui.elements["#updated"].dateTime, previousTime);
  assert.equal(ui.elements["#status"].dataset.state, "error");
  assert.equal(ui.elements["#error"].hidden, false);
  assert.match(ui.elements["#error"].textContent, /前回取得時/);
  connected = true;
  await ui.intervals[0].callback();
  assert.equal(ui.elements["#status"].dataset.state, "connected");
  assert.equal(ui.elements["#error"].hidden, true);
  assert.equal(ui.elements["#error"].textContent, "");
});

test("初回のHTTPエラーは未取得として表示し、成功値に見せない", async () => {
  const ui = createDashboard(async () => ({ ok: false, status: 503 }));
  await settle();
  assert.equal(ui.elements["#status"].textContent, "接続エラー");
  assert.match(ui.elements["#error"].textContent, /まだ取得できていません/);
  assert.equal(ui.elements["#updated"].dateTime, undefined);
  assert.equal(ui.timeouts.size, 0);
});

test("不正JSON・不正な使用率は表示せず、前回成功した値を保持する", async () => {
  let response = ok();
  const ui = createDashboard(async () => response);
  await settle();
  for (const invalid of [null, {}, { cpu_percent: "42.5", memory_percent: 61.2 }, { cpu_percent: -1, memory_percent: 61.2 }, { cpu_percent: 42.5, memory_percent: 101 }]) {
    response = { ok: true, json: async () => invalid };
    await ui.intervals[0].callback();
    assert.equal(ui.elements["#status"].dataset.state, "error");
    assert.equal(ui.elements["#cpu"].textContent, "42.5%");
    assert.equal(ui.elements["#memory"].textContent, "61.2%");
  }
  response = { ok: true, json: async () => { throw new SyntaxError("invalid JSON"); } };
  await ui.intervals[0].callback();
  assert.equal(ui.elements["#error"].hidden, false);
});

test("遅い通信を重複させず、4秒でタイムアウトした後も更新を再開できる", async () => {
  let calls = 0;
  const ui = createDashboard((url, { signal }) => {
    calls += 1;
    if (calls > 1) return Promise.resolve(ok());
    return new Promise((resolve, reject) => signal.addEventListener("abort", () => {
      const error = new Error("Aborted");
      error.name = "AbortError";
      reject(error);
    }));
  });
  await ui.intervals[0].callback();
  assert.equal(calls, 1);
  const timeout = [...ui.timeouts.values()][0];
  assert.equal(timeout.delay, 4000);
  timeout.callback();
  await settle();
  assert.match(ui.elements["#error"].textContent, /タイムアウト/);
  assert.equal(ui.timeouts.size, 0);
  await ui.intervals[0].callback();
  assert.equal(calls, 2);
  assert.equal(ui.elements["#status"].dataset.state, "connected");
});

test("境界値の0%と100%を正しく表示する", async () => {
  const ui = createDashboard(async () => ok(0, 100));
  await settle();
  assert.equal(ui.elements["#cpu"].textContent, "0.0%");
  assert.equal(ui.elements["#memory"].textContent, "100.0%");
  assert.equal(ui.elements["#memory-meter"].value, 100);
  assert.equal(ui.elements["#memory-meter"].attributes["aria-valuetext"], "100.0%");
});
