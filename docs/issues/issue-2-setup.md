# Issue #2：Rust開発環境とGitリポジトリの構築

- 対象：[Issue #2](https://github.com/rjc-4g/202610-team-b/issues/2)
- 記録日：2026-10-21
- 作業ブランチ：`feature/issue2`

## 目的と実装範囲

Rust + Axum + HTML/JavaScript + Docker Composeによるサーバー監視ダッシュボードの開発基盤を準備します。
このIssueではRustの実行プロジェクトを初期化し、Hello Worldの起動・テストと、PRレビューまでの開発手順を確認します。
AxumによるHTTPサーバー、監視画面、Docker Composeによるアプリ構成は後続Issueの対象です。

## 実装の過程

1. READMEとIssueの要件を確認しました。リポジトリは取得済みで、フォルダ構成と`/target/`の除外設定がありました。
2. 既存の`feature/issue2`ブランチを使用しました。
3. Rust / CargoがPATH上にないことを確認し、`Cargo.toml`と`src/main.rs`を追加して実行プロジェクトを初期化しました。`cargo init`は使用していません。
4. パッケージ名を`server-monitoring-dashboard`、Editionを`2024`とし、外部依存ライブラリなしで`Hello, world!`を表示する構成にしました。
5. 実ファイルを配置した`src/`から`.gitkeep`を削除しました。Cargoの実行で生成された`Cargo.lock`をGit管理対象に追加しました。
6. READMEに環境準備、起動・テスト、Dockerでの検証、開発の基本手順を記載しました。
7. ローカルで初期実装をコミットしました（`269229c`）。その後、実装経緯とIssue固有のレビュー事項をこのドキュメントに分離しました。

## 環境上の問題と対応

- GitとDocker CLI / Composeは利用できましたが、当初はDockerエンジンへ接続できませんでした。
- 検証用としてRust公式のWindows GNU版ツールチェーンを一時フォルダに導入しました。恒久的なPATHの変更は行っていません。
- GNU版では日本語を含むビルド出力パスでテストのリンクに失敗したため、`CARGO_TARGET_DIR`を一時フォルダに設定して検証しました。
- Docker起動後、Rust公式イメージでも実行とテストに成功しました。再現用コマンドは[READMEのDocker手順](../../README.md#dockerでの動作確認powershell)に記載しています。
- ホストWindowsの標準MSVC版Rust環境は未構築・未検証です。

## 検証結果（2026-10-21）

| 確認項目 | 環境・結果 |
| --- | --- |
| Git | 2.50.1.windows.1 |
| Docker Engine | 29.9.1、接続成功 |
| Docker Compose | v2.39.2-desktop.1 |
| `cargo run --offline --locked` | Dockerの`rust:1.99.1-slim`で成功。`Hello, world!`の出力を確認 |
| `cargo test --offline --locked` | 同イメージで成功。テストケースは0件で、テスト用ビルド・実行を確認 |
| `cargo fmt --check` | 一時導入したWindows GNU版Rust 1.99.1で成功 |
| `cargo clippy --offline --locked -- -D warnings` | 同GNU版環境で成功 |
| `git check-ignore target/` | 既存の除外ルールが有効であることを確認 |
| `git diff --check` | 初期実装の差分で成功 |

Hello Worldのみのため、機能テストはまだ追加していません。機能実装時に振る舞いを検証するテストを追加します。

## PR・レビューの状況（2026-10-21時点）

初期実装のローカルコミットは完了しています。公開リポジトリへのpushは自動承認レビューで明示的な承認不足を理由に拒否され、push・PR作成・担当者によるPRレビューは未実施です。

レビュー時には以下を確認します。

- READMEの手順でHello Worldを起動できること。
- `cargo test`が成功すること。
- `target/`がGit管理対象外であること。
- READMEの利用手順と`docs/`の作業記録が分かれていること。

Issue #2の完了条件にはPRレビューも含まれるため、現時点ではIssue全体の完了扱いにはしていません。

## ライブラリ利用ルールの追加と固定化（2026-10-21）

ユーザーの依頼に基づき、初回は`latest`相当で取得し、取得結果を固定する方針を追加しました。ルールはREADMEと[依存管理の手順](../dependencies.md)に記載し、AI向けの`AGENTS.md`にも反映しています。

今回の確認と変更は次のとおりです。

1. `Cargo.toml`と`Cargo.lock`を確認し、外部ライブラリが0件であることを確認しました。Axumなどは追加していません。
2. 一時導入したWindows GNU版に対して`rustup update stable --no-self-update`を実行しました。公式stableは`rustc 1.99.1`のままでした。
3. 実際の取得結果に基づき、`rust-toolchain.toml`の`channel`を`1.99.1`に固定しました。`rustfmt`と`clippy`も同ツールチェーンのコンポーネントとして指定しています。
4. `docker pull rust:latest`を実行し、コンテナ内でRust / Cargoがともに`1.99.1`であることを確認しました。
5. `docker image inspect rust:latest`の結果から、READMEのDocker手順を以下のdigestに固定しました。初期検証で使用した`rust:1.99.1-slim`から切り替えています。

```text
rust@sha256:7e9eb23ac694dda816c76933ab63b2703e6becbf3b10da90970124656788d47b
```

固定したDockerイメージと`rust-toolchain.toml`を使い、以下を再検証してすべて成功しました。

- `cargo run --offline --locked`：`Hello, world!`を表示。
- `cargo test --offline --locked`：成功（テストケース0件）。
- `cargo fmt --check`：成功。
- `cargo clippy --offline --locked -- -D warnings`：成功。

`Cargo.toml`と`Cargo.lock`の変更はありません。`version = "0.1.0"`はアプリ自身のバージョンであり、外部ライブラリのバージョンではありません。`edition = "2024"`はRustの言語Edition、lockファイルの`version = 4`はファイル形式のバージョンです。

GitとDocker Desktop / Engine自体の更新は行っていません。固定化の対象はプロジェクトで使用するRustツールチェーンとDockerイメージです。
