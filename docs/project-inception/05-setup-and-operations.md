# 05. セットアップと運用手順

以下は実装を進めるための手順であり、すべてを現在のmainで実行できるわけではない。2026-10-21の確認時点では、mainに`Cargo.toml`、API、UI、Docker設定、CIはまだない。各手順は対応Issueの実装後に利用する。

## 1. 前提条件

- GitHubリポジトリ`rjc-4g/202610-team-b`にアクセスできる
- Rust（rustup/Cargo）、Git、Docker EngineまたはDocker Desktop
- GitHub CLI（`gh`）、任意でVS Code + rust-analyzer、Codex CLI
- Windowsの場合はWSL2 + Docker Desktop連携を推奨。macOS/Linuxでも可。

**開発PCのOSは未確認。** OS固有のインストール手順は確定していない。

## 2. 既存リポジトリで開始

```bash
git clone https://github.com/rjc-4g/202610-team-b.git
cd 202610-team-b
ls -la
```

作業前に登録済みのSetup Issueと既存PR・ブランチを確認し、作業が進んでいる場合は担当者と共有して再利用する。新しく開始する場合はfeatureブランチを作る。

```bash
git switch -c feature/setup
```

既存の`Cargo.toml`がない場合に限り、リポジトリ直下で次を実行する。

```bash
cargo init --bin --name rust-system-monitor .
cargo run
cargo test
```

> `cargo init`が既存ファイルと衝突する場合は停止して構成を確認する。`Cargo.lock`はアプリケーションでは通常コミットする。`target/`は除外する。

## 3. ローカルAPI起動

API Issueの実装後に、リポジトリ直下で実行する。SetupのHello WorldだけではHTTPサーバーは起動しない。

```bash
cargo run
```

別のターミナル:

```bash
curl http://localhost:8080/api/health
curl http://localhost:8080/api/metrics
```

## 4. Docker起動

API・UI・Docker Issueの実装後に実行する。

```bash
docker compose up --build
```

`http://localhost:8080`を開く。停止は`Ctrl+C`または別ターミナルから`docker compose down`。

## 5. GitHub認証とPR

```bash
gh auth login
gh auth status
gh issue list --repo rjc-4g/202610-team-b
```

Projectの操作権限が足りない場合:

```bash
gh auth refresh -s project
```

## 6. よくある問題

| 症状 | 確認事項 |
|---|---|
| `cargo`が見つからない | rustupの導入、シェル再起動、PATH |
| ポート8080が使用中 | 既存プロセス/コンテナを停止する |
| Dockerビルド失敗 | Cargo.lock、パッケージ名、glibc互換性、Docker起動状態 |
| 画面は出るが値がない | `/api/metrics`の応答、ブラウザConsole、静的配信 |
| `gh issue create`で403 | ログインユーザーのリポジトリ権限、組織ポリシー |
| `gh project`で権限エラー | `project`スコープ、組織でのProject作成権限 |
| 実測値が読めない | cgroup v2、Docker DesktopのVM、ファイル権限 |

## 7. PRの確認観点

- `cargo fmt --all -- --check`が成功する
- `cargo clippy --all-targets -- -D warnings`が成功する
- `cargo test --locked`が成功する
- Dockerビルドが成功する
- エラー処理や`panic`の使い方に問題がない
- APIレスポンスやREADMEが仕様に一致する
- AIレビューの指摘を鵜呑みにせず、人間が採否を判断する
