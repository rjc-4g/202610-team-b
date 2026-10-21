# 03. Issue計画と工数

## 工数の全体像

| 順序 | 登録済みIssue | 開発 | 管理・レビュー | 合計 | 必須 |
|---:|---|---:|---:|---:|---|
| 1 | [#2](https://github.com/rjc-4g/202610-team-b/issues/2) [Setup] Rust開発環境とGitリポジトリの構築 | 1.0h | 1.0h | 2.0h | はい |
| 2 | [#3](https://github.com/rjc-4g/202610-team-b/issues/3) [API] Axumでダミー監視APIを実装 | 2.0h | 0.5h | 2.5h | はい |
| 3 | [#4](https://github.com/rjc-4g/202610-team-b/issues/4) [UI] サーバー監視ダッシュボードを実装 | 2.0h | 0.5h | 2.5h | はい |
| 4 | [#5](https://github.com/rjc-4g/202610-team-b/issues/5) [Docker] DockerfileとComposeで起動可能にする | 1.0h | 0.5h | 1.5h | はい |
| 5 | [#6](https://github.com/rjc-4g/202610-team-b/issues/6) [CI/CD] GitHub Actionsで自動テストとDockerビルド | 2.0h | 1.0h | 3.0h | はい |
| 6 | [#7](https://github.com/rjc-4g/202610-team-b/issues/7) [Challenge] cgroup v2でCPU・メモリ実測値を取得 | 1.5h | 0.5h | 2.0h | いいえ |
| 7 | [#8](https://github.com/rjc-4g/202610-team-b/issues/8) [Review] AIコードレビュー・README・成果発表 | 0.5h | 1.0h | 1.5h | はい |
| | **合計** | **10.0h** | **5.0h** | **15.0h** | |

> Issue番号はリポジトリで既に使われている場合があるため、上記の「順序」とGitHub Issue番号は別物として扱う。

2026-10-21にGitHubから取得したIssue番号・URLを記載している。以下は計画上のチェックリストであり、最新の完了状況は各Issueと[Board](https://github.com/orgs/rjc-4g/projects/7/views/2)で管理する。追加課題を除く必須部分は13人時（開発8.5、管理4.5）で、追加チャレンジより先にレビュー・成果発表の時間を確保する。

## 計画1: 環境構築（Issue #2）

**目的:** Rustの起動、Gitの基本操作、PRレビューを体験する。

- [ ] Rust/Cargo、Git、Dockerを準備する
- [ ] 既存リポジトリをcloneし、Rustプロジェクトを初期化する
- [ ] `cargo run`でHello Worldを表示する
- [ ] `cargo test`が成功する
- [ ] `target/`をGit管理対象外にする
- [ ] READMEを作成し、PRレビューを実施する

## 計画2: Axum API（Issue #3）

**目的:** RustでJSON APIを作成する。

- [ ] `GET /api/health` → `{"status":"ok"}`
- [ ] `GET /api/metrics` → `{"cpu_percent":42.5,"memory_percent":61.2}`
- [ ] 8080番ポートで起動し、curlで動作確認する
- [ ] `cargo fmt`、`cargo test`が成功する
- [ ] PRレビューを実施する

## 計画3: ダッシュボードUI（Issue #4）

**目的:** APIデータをブラウザに表示する。

- [ ] HTML/CSS/JSの静的ファイルを作る
- [ ] CPUとメモリを表示する
- [ ] 5秒間隔で更新する
- [ ] 通信エラー時に状態を表示する
- [ ] （追加）履歴グラフを表示する

## 計画4: Docker（Issue #5）

**目的:** Composeで再現可能な実行環境を作る。

- [ ] マルチステージDockerfileを作る
- [ ] `compose.yaml`を作る
- [ ] `docker compose up --build`で起動する
- [ ] 画面とAPIが利用できる
- [ ] READMEに起動・停止手順を書く

## 計画5: CI/CD（Issue #6）

**目的:** PR時の自動検証と失敗→修正を体験する。

- [ ] GitHub ActionsをPR/main pushで起動する
- [ ] `cargo fmt --all -- --check`が成功する
- [ ] `cargo clippy --all-targets -- -D warnings`が成功する
- [ ] `cargo test --locked`が成功する
- [ ] `docker build`が成功する
- [ ] CI失敗から修正までの履歴を残す
- [ ] （追加）GHCRへのイメージ公開

## 計画6: cgroup v2（チャレンジ、Issue #7）

**目的:** コンテナのCPU・メモリ実測値をAPIに反映する。

- [ ] cgroup v2利用可否を確認する
- [ ] `memory.current`、`memory.max`を読み取る
- [ ] `cpu.stat`の差分からCPU使用率を計算する
- [ ] CPU使用率の基準をREADME・UIに明記する
- [ ] 無制限・初回計測・読み取り失敗を処理する
- [ ] 計算の単体テストを作る
- [ ] コンテナを測定していることをREADMEに明記する

**注:** このIssueは未完了でもMVPは完成扱いとする。

## 計画7: レビューと成果発表（Issue #8）

**目的:** AIと人間のレビューを体験し、成果を記録する。

- [ ] AIにPR差分のレビューを依頼する
- [ ] AI指摘を人間が確認し、採否を記録する
- [ ] 管理担当がレビューを実施する
- [ ] READMEに概要・起動・API・制約を書く
- [ ] Docker・API・UI・CIのデモを実施する
- [ ] 学習事項と改善点をまとめる

## 担当ルール

開発担当が実装・動作確認・PR作成を担当し、管理担当がIssue管理・受け入れ確認・PRレビューを担当する。AIは補助であり、最終判断は人間が行う。
