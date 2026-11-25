"use strict";

// ブラウザ専用の設定値。Rust側の定数とは実行環境が異なるため、ここに集約する。
const REFRESH_INTERVAL_MS = 5000;
const REQUEST_TIMEOUT_MS = 4000;
const METRICS_URL = "/api/metrics";

const cpu = document.querySelector("#cpu");
const memory = document.querySelector("#memory");
const cpuMeter = document.querySelector("#cpu-meter");
const memoryMeter = document.querySelector("#memory-meter");
const status = document.querySelector("#status");
const updated = document.querySelector("#updated");
const errorMessage = document.querySelector("#error");
let requestInProgress = false;
let hasSuccessfulUpdate = false;

// 引数の値がMVPの使用率（有限の数値、0〜100%）として有効かを返す。
// nullや数値の文字列を受け入れず、API異常を0%などとして表示しない。
function isPercentage(value) {
  return typeof value === "number" && Number.isFinite(value) && value >= 0 && value <= 100;
}

// APIを取得して画面を更新する。引数はなく、完了を表すPromiseを返す。
// awaitで通信を待っている間も、ブラウザの描画や操作は継続できる。
async function refreshMetrics() {
  // 遅い通信中に次の更新時刻になっても、リクエストを重複させない。
  if (requestInProgress) return;
  requestInProgress = true;
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);

  try {
    // 相対URLで同じサーバーへ問い合わせるため、Dockerでも接続先の変更は不要。
    const response = await fetch(METRICS_URL, { signal: controller.signal, cache: "no-store" });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const metrics = await response.json();
    if (!metrics || !isPercentage(metrics.cpu_percent) || !isPercentage(metrics.memory_percent)) {
      throw new Error("監視APIの応答形式が不正です");
    }

    // 両方の値を検証してからまとめて更新し、不正な応答で一部だけ変わることを防ぐ。
    cpu.textContent = `${metrics.cpu_percent.toFixed(1)}%`;
    memory.textContent = `${metrics.memory_percent.toFixed(1)}%`;
    cpuMeter.value = metrics.cpu_percent;
    memoryMeter.value = metrics.memory_percent;
    cpuMeter.setAttribute("aria-valuetext", cpu.textContent);
    memoryMeter.setAttribute("aria-valuetext", memory.textContent);
    const now = new Date();
    updated.textContent = now.toLocaleString("ja-JP");
    updated.dateTime = now.toISOString();
    hasSuccessfulUpdate = true;
    status.textContent = "接続済み";
    status.dataset.state = "connected";
    errorMessage.hidden = true;
    errorMessage.textContent = "";
  } catch (error) {
    status.textContent = "接続エラー";
    status.dataset.state = "error";
    const detail = error.name === "AbortError" ? "応答がタイムアウトしました。" : "監視データを取得できませんでした。";
    const previous = hasSuccessfulUpdate ? "表示値と最終更新日時は前回取得時のものです。" : "監視値はまだ取得できていません。";
    errorMessage.textContent = `${detail}${previous}5秒ごとに再接続します。`;
    errorMessage.hidden = false;
  } finally {
    // 成功・失敗のどちらでもタイマーと通信中フラグを解除し、次回の取得を可能にする。
    clearTimeout(timeout);
    requestInProgress = false;
  }
}

// 初回はすぐに取得し、その後は5秒ごとに再取得する。失敗後も自動で復帰を試みる。
refreshMetrics();
setInterval(refreshMetrics, REFRESH_INTERVAL_MS);
