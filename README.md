# サーバー監視ダッシュボード

Rust + Axum + HTML/JavaScript + Docker Composeで開発するサーバー監視ダッシュボードです。
まずはダミー値でCPU・メモリ使用率を表示するMVPを完成させ、コンテナの実測値取得は追加チャレンジとします。

## 現在の実装

[Issue #2](https://github.com/rjc-4g/202610-team-b/issues/2)として、Rustの実行プロジェクトを初期化しています。
現在は`cargo run`で`Hello, world!`を表示して終了します。AxumによるHTTPサーバー、監視画面、Docker Compose構成は今後の実装対象です。

## 開発環境

- Rust / Cargo（[rustup公式サイト](https://rust-lang.org/tools/install/)からstable版をインストール）
- Git
- Docker / Docker Compose v2（今後のコンテナ実行で使用）

RustはEdition 2024を使用します。Windowsの標準構成（MSVC版）では、Rustのインストール時に案内されるVisual Studio C++ Build Toolsも必要です。
インストール後にターミナルを開き直し、以下を確認してください。

```sh
rustc --version
cargo --version
git --version
docker --version
docker compose version
docker info
```

`docker info`で接続エラーが出る場合はDocker DesktopなどのDockerエンジンを起動してください。
現在のHello Worldの実行・テストにはDockerは不要です。

## 起動・テスト

初めて取得する場合は、リポジトリをcloneします。取得済みの場合はそのフォルダに移動してください。

```sh
git clone https://github.com/rjc-4g/202610-team-b.git
cd 202610-team-b
```

Rustプロジェクトは初期化済みのため、`cargo init`の再実行は不要です。

```sh
cargo run
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

`cargo run`は`Hello, world!`を表示して自動終了するため、停止操作は不要です。
`cargo test`は現在テストケースが0件の状態で、テスト用のビルドと実行が成功することを確認します。
機能を追加する際に、その振る舞いを検証するテストも追加します。

ビルド生成物の`target/`は`.gitignore`で除外しています。
アプリケーションの依存バージョンを共有するため、`Cargo.lock`はGitで管理します。

### Dockerでの動作確認（PowerShell）

ホスト側にRustをインストールしていない場合は、Dockerエンジンを起動し、リポジトリのルートで以下を実行できます。

```powershell
docker run --rm --mount "type=bind,source=$($PWD.Path),target=/app,readonly" --workdir /app --env CARGO_TARGET_DIR=/tmp/target rust:1.99.1-slim sh -c 'cargo run --offline --locked && cargo test --offline --locked'
```

初回はRust公式イメージのダウンロードにネットワーク接続が必要です。
ソースは読み取り専用でマウントし、ビルド生成物はコンテナ内の一時フォルダに出力します。
コマンド終了時にコンテナとそのビルド生成物は削除されます。これは現在の依存ライブラリなしのHello World用の確認手順です。

## フォルダ構成

```text
.
├── .github/
│   └── workflows/  # GitHub ActionsのCI設定
├── docs/           # 設計・作業手順・レビュー・成果発表のドキュメント
├── src/
│   └── main.rs     # Hello Worldのエントリーポイント
├── static/         # HTML・CSS・JavaScript
├── .gitignore
├── Cargo.toml      # Rustプロジェクトの定義
├── Cargo.lock      # 依存バージョンの固定
└── README.md
```

未実装の空フォルダには、Gitで管理するために`.gitkeep`を配置しています。実ファイルを追加したら削除できます。

## 開発の進め方

1. 対応する[Issue](https://github.com/rjc-4g/202610-team-b/issues)を確認する。
2. 作業用ブランチを作成する（例：`git switch -c feature/issue2`）。既存の作業用ブランチがある場合は切り替えて使う。
3. 実装後に`cargo run`、`cargo test`、フォーマット・静的解析の確認を実施する。
4. 変更をcommit・pushし、Pull Requestを作成する。PR本文に対象Issue、変更点、確認結果、未確認事項を記載する。
5. 管理・レビュー担当が差分と動作を確認し、指摘への対応後にマージする。

Issue #2のレビューでは、Hello Worldの起動、`cargo test`の成功、`target/`の除外、READMEの手順を確認してください。
`git check-ignore target/`で除外ルールを確認できます。

ドキュメントの格納方針は[docs/README.md](docs/README.md)を参照してください。

## Author
[株式会社ＲＪＣ](https://www.r-jc.jp/) システム開発部第4グループ
