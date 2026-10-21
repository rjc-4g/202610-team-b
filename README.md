# サーバー監視ダッシュボード

Rust + Axum + HTML/JavaScript + Docker Composeで開発するサーバー監視ダッシュボードです。
まずはダミー値でCPU・メモリ使用率を表示するMVPを完成させ、コンテナの実測値取得は追加チャレンジとします。

## 開発環境

- Rust / Cargo（[rustup公式サイト](https://rust-lang.org/tools/install/)からrustupを導入し、`rust-toolchain.toml`に記録された固定版を使用）
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

```sh
cargo run --locked
cargo test --locked
cargo fmt --check
cargo clippy --locked -- -D warnings
```

`cargo run`は`Hello, world!`を表示して自動終了するため、停止操作は不要です。
`cargo test`は現在テストケースが0件の状態で、テスト用のビルドと実行が成功することを確認します。
機能を追加する際に、その振る舞いを検証するテストも追加します。

ビルド生成物の`target/`は`.gitignore`で除外しています。
アプリケーションの依存バージョンを共有するため、`Cargo.lock`はGitで管理します。

### Dockerでの動作確認（PowerShell）

ホスト側にRustをインストールしていない場合は、Dockerエンジンを起動し、クローンしたリポジトリのルートフォルダ（Cargo.tomlがあるフォルダ）に移動して、PowerShellで以下を実行してください。

```powershell
docker run --rm --mount "type=bind,source=$($PWD.Path),target=/app,readonly" --workdir /app --env CARGO_TARGET_DIR=/tmp/target rust@sha256:7e9eb23ac694dda816c76933ab63b2703e6becbf3b10da90970124656788d47b sh -c 'cargo run --offline --locked && cargo test --offline --locked'
```

Rust公式イメージは`latest`から取得して確認したdigestで固定しています。
イメージの初回取得と、`rust-toolchain.toml`で指定した未導入コンポーネントの取得にはネットワーク接続が必要です（`--offline`はCargoの依存解決に対する指定です）。
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
├── rust-toolchain.toml # Rust / Cargoのバージョン固定
└── README.md
```

未実装の空フォルダには、Gitで管理するために`.gitkeep`を配置しています。実ファイルを追加したら削除できます。

## 開発の進め方

1. 対応する[Issue](https://github.com/rjc-4g/202610-team-b/issues)を確認する。
2. 対応するIssueの作業用ブランチを作成する。既存の作業用ブランチがある場合は切り替えて使う。
3. 実装後に`cargo run --locked`、`cargo test --locked`、フォーマット・静的解析の確認を実施する。
4. 変更をcommit・pushし、Pull Requestを作成する。PR本文に対象Issue、変更点、確認結果、未確認事項を記載する。
5. 管理・レビュー担当が差分と動作を確認し、指摘への対応後にマージする。

Issueごとの実装経緯・検証結果・レビュー事項は`docs/issues/`に記録します。
ドキュメント一覧と格納方針は[docs/README.md](docs/README.md)を参照してください。

## ライブラリ利用ルール

- 初回インストール時は `latest` を使用する。
- AI側で勝手にバージョン番号を判断して直打ちしない。
- インストール後、`package.json` と lock ファイルに記録されたバージョンを固定版として扱う。
- 実装中に勝手にライブラリを追加・更新しない。
- ライブラリ追加が必要な場合は、理由を説明してユーザ承認を得る。

Rustでは`Cargo.toml`・`Cargo.lock`と`rust-toolchain.toml`で管理します。最新取得と固定の具体的な手順は[ライブラリ利用ルールの適用方法](docs/dependencies.md)を参照してください。

## Author
[株式会社ＲＪＣ](https://www.r-jc.jp/) システム開発部第4グループ
