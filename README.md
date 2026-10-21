# サーバー監視ダッシュボード

Rust + Axum + HTML/JavaScript + Docker Composeで開発するサーバー監視ダッシュボードです。
まずはダミー値でCPU・メモリ使用率を表示するMVPを完成させ、コンテナの実測値取得は追加チャレンジとします。
現在はAxumによるヘルスチェックとダミー監視APIを実装しています。監視画面は未実装です。

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
ホスト側にRust環境がある場合、APIの実行・テストにDockerは不要です。

## 起動・テスト

初めて取得する場合は、リポジトリをcloneします。取得済みの場合はそのフォルダに移動してください。

```sh
git clone https://github.com/rjc-4g/202610-team-b.git
cd 202610-team-b
```

```sh
cargo run --locked
```

起動後、`http://localhost:8080`でAPIを利用できます。停止するには実行中のターミナルで`Ctrl+C`を押します。
コンテナ内からのポート公開にも対応するため、アプリは`0.0.0.0:8080`（全IPv4インターフェース）で待ち受けます。
ポート8080が使用中の場合は、使用しているプロセスを確認し、必要に応じて停止してから起動してください。

別のターミナルで動作確認します。PowerShellでは`curl`のエイリアスと区別するため`curl.exe`を使用してください。

```sh
curl --fail-with-body -i http://localhost:8080/api/health
curl --fail-with-body -i http://localhost:8080/api/metrics
```

| メソッド・パス | 応答（HTTP 200、Content-Type: application/json） |
| --- | --- |
| `GET /api/health` | `{"status":"ok"}` |
| `GET /api/metrics` | `{"cpu_percent":42.5,"memory_percent":61.2}` |

監視値は固定のダミー値であり、実際のCPU・メモリ使用率は取得しません。未定義のパスは404、両APIへのPOSTは405を返します。

テストと静的解析は次のコマンドで実行します。

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked -- -D warnings
```

`cargo test`は一時的なローカルHTTPサーバーを使い、両APIのJSON・Content-Type・200応答と404・405応答を検証します（4件）。ポートはOSが自動割り当てするため、起動済みのアプリとも併用できます。

ビルド生成物の`target/`は`.gitignore`で除外しています。
アプリケーションの依存バージョンを共有するため、`Cargo.lock`はGitで管理します。

### Dockerでの動作確認（PowerShell）

ホスト側にRustをインストールしていない場合は、Dockerエンジンを起動し、クローンしたリポジトリのルートフォルダ（Cargo.tomlがあるフォルダ）に移動して、PowerShellで以下を実行してください。

```powershell
docker run --rm --name monitoring-api --publish 127.0.0.1:8080:8080 --mount "type=bind,source=$($PWD.Path),target=/app,readonly" --workdir /app --env CARGO_TARGET_DIR=/tmp/target rust@sha256:7e9eb23ac694dda816c76933ab63b2703e6becbf3b10da90970124656788d47b cargo run --locked
```

別のターミナルから上記の`curl.exe`で確認できます。終了するには別のターミナルで`docker stop monitoring-api`を実行します。

テスト・フォーマット確認・静的解析は以下で実行します。

```powershell
docker run --rm --mount "type=bind,source=$($PWD.Path),target=/app,readonly" --workdir /app --env CARGO_TARGET_DIR=/tmp/target rust@sha256:7e9eb23ac694dda816c76933ab63b2703e6becbf3b10da90970124656788d47b sh -c 'cargo test --locked && cargo fmt --check && cargo clippy --locked -- -D warnings'
```

Rust公式イメージは`latest`から取得して確認したdigestで固定しています。
イメージの初回取得、`rust-toolchain.toml`で指定した未導入コンポーネント、Cargo依存ライブラリの取得にはネットワーク接続が必要です。
ソースは読み取り専用でマウントし、ビルド生成物はコンテナ内の一時フォルダに出力します。
コンテナ終了時にコンテナとそのビルド生成物・依存キャッシュは削除されるため、再実行時には依存ライブラリを再取得します。

## フォルダ構成

```text
.
├── .github/
│   └── workflows/  # GitHub ActionsのCI設定
├── docs/           # 設計・作業手順・レビュー・成果発表のドキュメント
├── src/
│   └── main.rs     # 監視APIのエントリーポイントとHTTPテスト
├── static/         # HTML・CSS・JavaScript
├── .gitignore
├── Cargo.toml      # Rustプロジェクトの定義
├── Cargo.lock      # 依存バージョンの固定
├── rust-toolchain.toml # Rust / Cargoのバージョン固定
└── README.md
```

未実装の空フォルダには、Gitで管理するために`.gitkeep`を配置しています。実ファイルを追加したら削除できます。

## ドキュメント

詳細は[ドキュメント一覧](docs/README.md)を参照してください。

## Author
[株式会社ＲＪＣ](https://www.r-jc.jp/) システム開発部第4グループ
