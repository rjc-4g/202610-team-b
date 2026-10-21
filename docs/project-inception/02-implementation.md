# 02. 実装ガイド

## 1. ディレクトリ例

```text
202610-team-b/
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs
├── static/
│   ├── index.html
│   ├── style.css
│   └── app.js
├── Dockerfile
├── .dockerignore
├── compose.yaml
├── .github/workflows/ci.yml
└── docs/project-inception/...
```

> 既存リポジトリに合わせて調整すること。`cargo new`で別のGitリポジトリを入れ子にしない。

## 2. 依存関係

```bash
cargo add axum
cargo add tokio --features full
cargo add serde --features derive
cargo add tower-http --features fs
```

## 3. 最小API例（`src/main.rs`）

```rust
use axum::{routing::get, Json, Router};
use serde::Serialize;
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct Metrics {
    cpu_percent: f64,
    memory_percent: f64,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

async fn metrics() -> Json<Metrics> {
    Json(Metrics {
        cpu_percent: 42.5,
        memory_percent: 61.2,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/metrics", get(metrics))
        .fallback_service(ServeDir::new("static").append_index_html_on_directories(true));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind port");
    println!("Server: http://localhost:8080");
    axum::serve(listener, app).await.expect("server failed");
}
```

期待値:

```json
{"status":"ok"}
```

```json
{"cpu_percent":42.5,"memory_percent":61.2}
```

## 4. UIの最低要件

`static/index.html`にCPU、メモリ、最終更新日時、接続状態を表示する領域を設ける。`static/app.js`では以下を実装する。

```javascript
async function refreshMetrics() {
  try {
    const response = await fetch('/api/metrics');
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const metrics = await response.json();
    document.querySelector('#cpu').textContent = `${metrics.cpu_percent}%`;
    document.querySelector('#memory').textContent = `${metrics.memory_percent}%`;
    document.querySelector('#status').textContent = 'Connected';
    document.querySelector('#updated').textContent = new Date().toLocaleString();
  } catch (error) {
    document.querySelector('#status').textContent = 'Disconnected';
    console.error(error);
  }
}
refreshMetrics();
setInterval(refreshMetrics, 5000);
```

HTML側に`id="cpu"`、`id="memory"`、`id="status"`、`id="updated"`の要素を用意する。

## 5. Docker例

`Dockerfile`:

```dockerfile
FROM rust:slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --locked

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-system-monitor /app/rust-system-monitor
COPY --from=builder /app/static ./static
EXPOSE 8080
CMD ["./rust-system-monitor"]
```

> 実際のCargoパッケージ名に応じて実行ファイル名を変更する。この例ではビルダーと実行環境のDebian系統をbookwormに揃えている。`Cargo.lock`を生成・コミットしてから`--locked`でビルドする。Rustタグは更新されるため、採用バージョンを固定する場合はCIと合わせて調整する。

`compose.yaml`:

```yaml
services:
  monitor:
    build: .
    ports:
      - "127.0.0.1:8080:8080"
    restart: unless-stopped
```

`.dockerignore`:

```text
.git
/target
.env
```

起動:

```bash
docker compose up --build
```

## 6. cgroup v2実測値（追加課題）

- メモリ: `/sys/fs/cgroup/memory.current`、`memory.max`。上限が正の数値なら`memory.current / memory.max × 100`で使用率を計算する。`max`はそのcgroupに上限設定がないため、この値を分母にしたパーセントは計算できない。
- CPUの計算例: `Δusage_usec / Δ経過時間_usec × 100`。単調増加時計で経過時間を測り、**1 CPUを使い切った状態を100%**とするため、複数CPUを使えば100%を超える。CPUクォータで正規化する場合は`cpu.max`のquota/periodを基準にする。採用する基準を実装時にREADME・UIに明記する。
- 最初の測定ではCPU差分が得られないため、準備中表示などを設ける。
- 経過時間が0の場合や、再起動などで累積CPU時間が減少した場合は計算せず、基準値を取り直す。
- cgroupのマウント位置、権限、Docker DesktopのVM構成によって動作が異なる。
- **これはコンテナのメトリクスであり、ホストPC全体の監視ではない。**
- 取得できない場合は安全にエラーを返すか、MVPの固定値モードを維持する。

固定値モードを使用する場合はUIにもダミー値であることを表示し、実測値と混同させない。取得不能・準備中をAPIでどう返すかは、UIとの整合を含めてチャレンジ着手時に決める。

## 7. テスト方針

APIのJSON形状、数値計算、`memory.max=max`、ファイル読み取り失敗をテストする。最初は`cargo test`が通る最小構成でよい。

## 8. 技術資料

- [Axum公式ドキュメント](https://docs.rs/axum/latest/axum/)
- [tower-http ServeDir](https://docs.rs/tower-http/latest/tower_http/services/struct.ServeDir.html)
- [Rust公式Dockerイメージのタグ定義](https://github.com/docker-library/official-images/blob/master/library/rust)
- [Linux Kernel: cgroup v2](https://docs.kernel.org/admin-guide/cgroup-v2.html)
