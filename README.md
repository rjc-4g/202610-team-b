# サーバー監視ダッシュボード

Rust + Axum + HTML/JavaScript + Docker Composeで開発するサーバー監視ダッシュボードです。
まずはダミー値でCPU・メモリ使用率を表示するMVPを完成させ、コンテナの実測値取得は追加チャレンジとします。
現在はAxumによるヘルスチェック・ダミー監視APIと、CPU・メモリ使用率を5秒ごとに更新する監視画面を実装しています。

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

起動後、ブラウザで`http://localhost:8080`を開くと監視画面を利用できます。停止するには実行中のターミナルで`Ctrl+C`を押します。
コンテナ内からのポート公開にも対応するため、アプリは`0.0.0.0:8080`（全IPv4インターフェース）で待ち受けます。
ポート8080が使用中の場合は、使用しているプロセスを確認し、必要に応じて停止してから起動してください。

画面にはCPU使用率42.5%、メモリ使用率61.2%、接続状態、最終更新日時を表示します。初回はすぐに取得し、その後は5秒ごとに更新します。値は固定のため、更新の確認には最終更新日時を見てください。

通信失敗・4秒のタイムアウト・不正なAPI応答ではエラーを表示します。取得済みの値と日時は前回のものとして残し、5秒ごとに再接続します。初回から失敗した場合は値を「—」、日時を「未取得」と表示します。

画面はJavaScriptが有効なブラウザで利用してください。HTML・CSS・JavaScriptはビルド時に実行ファイルへ埋め込むため、`static/`を変更した後は`cargo run --locked`などで再ビルド・再起動し、ブラウザを再読み込みします。実行時の作業フォルダに`static/`を置く必要はありません。

APIは別のターミナルで動作確認できます。PowerShellでは`curl`のエイリアスと区別するため`curl.exe`を使用してください。

```sh
curl --fail-with-body -i http://localhost:8080/api/health
curl --fail-with-body -i http://localhost:8080/api/metrics
```

| メソッド・パス | 応答（HTTP 200、Content-Type: application/json） |
| --- | --- |
| `GET /api/health` | `{"status":"ok"}` |
| `GET /api/metrics` | `{"cpu_percent":42.5,"memory_percent":61.2}` |

監視値は固定のダミー値であり、実際のCPU・メモリ使用率は取得しません。画面とAPIの未定義パスは404、公開パスへのPOSTは405を返します。

テストと静的解析は次のコマンドで実行します。

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked -- -D warnings
```

`cargo test`は一時的なローカルHTTPサーバーを使い、両APIのJSON・Content-Type・200応答、画面と静的資産の配信、404・405応答を検証します（6件）。ポートはOSが自動割り当てするため、起動済みのアプリとも併用できます。

Node.jsが導入済みの場合は、画面処理の単体テストも実行できます（追加パッケージ不要、Node.js v24.6.0で検証）。Node.jsはアプリの起動には不要です。

```sh
node --check static/app.js
node --test tests/dashboard.test.cjs
```

初回取得・5秒間隔の更新、通信失敗と復帰、HTTPエラー、不正な応答、タイムアウトと重複通信防止、0%と100%の表示を検証します（6件）。DOM・通信・タイマーを代替した単体テストのため、レイアウトはブラウザで別途確認します。

画面の手動確認は、サーバー起動後に以下を実施してください。

1. `http://localhost:8080`でCPU・メモリ値、接続済み、ダミーデータの表示を確認する。
2. 5秒以上待ち、最終更新日時が変わることを確認する。
3. ページを開いたままサーバーを停止し、接続エラーと前回値である旨の表示を確認する。
4. サーバーを再起動し、再読み込みなしで接続済みに戻ることを確認する。
5. ブラウザ幅を狭め、カードが縦に並び、横にはみ出さないことを確認する。

ビルド生成物の`target/`は`.gitignore`で除外しています。
アプリケーションの依存バージョンを共有するため、`Cargo.lock`はGitで管理します。

### Dockerでの動作確認（PowerShell）

ホスト側にRustをインストールしていない場合は、Dockerエンジンを起動し、クローンしたリポジトリのルートフォルダ（Cargo.tomlがあるフォルダ）に移動して、PowerShellで以下を実行してください。

```powershell
docker run --rm --name monitoring-api --publish 127.0.0.1:8080:8080 --mount "type=bind,source=$($PWD.Path),target=/app,readonly" --workdir /app --env CARGO_TARGET_DIR=/tmp/target rust@sha256:7e9eb23ac694dda816c76933ab63b2703e6becbf3b10da90970124656788d47b cargo run --locked
```

ブラウザで`http://localhost:8080`を開くと画面を確認でき、別のターミナルから上記の`curl.exe`でAPIも確認できます。終了するには別のターミナルで`docker stop monitoring-api`を実行します。

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
│   ├── constants.rs # 待ち受け設定と埋め込み配信資産
│   └── main.rs     # 画面・監視APIのエントリーポイントとHTTPテスト
├── static/         # HTML・CSS・JavaScript（ビルド時に埋め込み）
├── tests/          # JavaScriptの画面処理テスト
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
