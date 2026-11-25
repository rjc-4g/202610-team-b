# Issue #5：DockerfileとComposeで起動可能にする

- 対象：[Issue #5](https://github.com/rjc-4g/202610-team-b/issues/5)
- 記録日：2026-11-25
- 作業ブランチ：`feature/issue5`

## 参照した資料と前提

- [開発ルール](../development-rules.md)、[ライブラリ利用ルール](../dependencies.md)、[README](../../README.md)
- [実装ガイド](../project-inception/02-implementation.md)
- [Issue #4の作業記録](issue-4-dashboard.md)と、そこから参照するIssue #2・#3の検証方式

開始時の`feature/issue5`は`15d0fdd`で、監視画面を含んでいませんでした。作業ツリーに変更がないことを確認し、`git merge --ff-only feature/issue4`で実装済みの`3d46d5a`を取り込みました。Issue #4のコードや依存関係は変更していません。

## 実装と判断

1. `Dockerfile`を追加しました。ビルドステージでは既存の固定済みRust公式イメージを使用し、`cargo build --release --locked`を実行します。`rust-toolchain.toml`と`Cargo.lock`を含めてビルドします。
2. `src/`と`static/`をビルドステージへコピーします。Issue #4は静的ファイルを実行ファイルへ埋め込むため、実行ステージに`static/`を配置する必要はありません。
3. 実行ステージは空の`scratch`から作り、アプリの実行ファイルと、ビルド環境の`ldd`で確認した共有ライブラリ・ローダーだけをコピーします。異なるOSイメージとのライブラリ互換性問題を避け、既存の固定イメージを再利用する構成です。別のベースイメージやパッケージは取得していません。
4. 共有ライブラリのパスは`ldd`の出力から抽出し、元の絶対パスを保ってコピーします。ライブラリ名やCPUアーキテクチャ依存のパスを直書きしていません。
5. 実行ユーザーは`65532:65532`で、root権限を使用しません。exec形式の`ENTRYPOINT`でアプリを直接起動します。
6. `compose.yaml`に`monitor`サービスを追加しました。実行ステージをビルドし、要件どおり`127.0.0.1:8080:8080`で公開します。コンテナ内の待ち受けはIssue #4の`0.0.0.0:8080`を維持しています。
7. `.dockerignore`はビルドに必要なファイルのみ許可する形式にしました。Git履歴、ホストのビルド生成物、`.env`、ローカル設定、ドキュメントは送信対象外です。
8. READMEにComposeでの前面・バックグラウンド起動、状態・ログ確認、停止、再ビルド、ポート競合時の対処を追加しました。従来のRust開発用`docker run`と、今回のComposeによるアプリ起動を分けて説明しています。

## 検証結果（2026-11-25）

検証環境はWindows上のDocker Desktop（Linuxコンテナ）、Docker Engine 29.9.1、Docker Compose v5.6.0、linux/amd64です。

開始時にユーザーの`monitoring-api`がホスト8080番で稼働していたため、そのコンテナは停止していません。標準Composeの公開先を`docker compose config`で確認したうえで、実行検証は一時的な上書きファイルと別プロジェクト名`issue5-qa`を使い、`127.0.0.1:18080:8080`で行いました。標準の8080番を使ったCompose起動は、この作業では未実施です。

一時ファイルはリポジトリ外に作成し、以下のようにポート設定を置換しました。`!override`は今回の検証用Composeで利用できることを確認しています。通常利用の`compose.yaml`には使用していません。

```yaml
services:
  monitor:
    ports: !override
      - "127.0.0.1:18080:8080"
```

| 確認項目 | 結果 |
| --- | --- |
| `docker compose config --quiet` / `docker compose config` | 成功。標準設定のhost_ipは127.0.0.1、公開先8080、コンテナ側8080 |
| `docker compose -p issue5-qa -f compose.yaml -f <一時ファイル> up --build -d` | マルチステージビルドと起動に成功 |
| 画面と静的資産 | `/`、`/style.css`、`/app.js`が200、HTML・CSS・JavaScriptのContent-Typeを確認 |
| API | `/api/health`が`{"status":"ok"}`、`/api/metrics`がCPU 42.5・メモリ61.2を返すことを確認 |
| Chromeでの確認 | 実行用コンテナから画面を開き、CPU 42.5%、メモリ61.2%、接続済み、最終更新日時を確認。スクリーンショットでも表示を確認 |
| 実行設定 | 非rootのUID/GID、ホスト127.0.0.1への公開、ソースのマウントなしを確認 |
| 実行用イメージ | `docker image inspect`のSizeは7,685,630 bytes（約7.7MB） |
| `down`と再起動 | 検証プロジェクトのコンテナ・ネットワーク削除、`up -d`での再作成、API正常応答を確認 |
| `cargo run --locked` | ビルドステージのイメージから18081番で起動し、ヘルスチェック成功 |
| `cargo test --locked` | 同ビルドステージで6件成功 |
| `cargo fmt --check` | 成功 |
| `cargo clippy --locked --all-targets -- -D warnings` | テストコードを含め成功 |
| 依存・アプリ差分 | Cargo.toml、Cargo.lock、rust-toolchain.toml、src/、static/にIssue #4からの変更なし |
| `git diff --check` | 成功 |

Cargoの検証用イメージは`docker build --target builder --tag server-monitor-issue5-check .`で作成しました。JavaScriptは変更していないため単体テストを再実行せず、今回の実行用コンテナでブラウザ表示を確認しました。

## 制約と後片付け

- `scratch`の実行用イメージにはシェル・Cargo・curlがありません。状態は`docker compose ps`、ログは`docker compose logs monitor`、APIはホストのブラウザやcurlから確認します。
- 現在のMVPに必要な実行ライブラリだけを含めています。将来、外部HTTPS通信・証明書・追加のOS機能を使う場合は、実行環境の構成も見直します。
- WindowsコンテナとARM64環境は未検証です。
- 通常のコマンド実行環境の既知の起動エラーを避け、許可されたサンドボックス外実行で検証しました。Dockerやライブラリのインストール・更新は行っていません。
- 今回作成した検証コンテナとComposeネットワークは停止・削除済みです。既存の`monitoring-api`は維持しています。ローカルのビルド済みイメージとビルドキャッシュは残しています。

## PR・レビュー

開発ルールに従い、ローカルコミットまでを実施します。push・PR作成・IssueやProjectの更新は行いません。管理・レビュー担当者の確認は未実施です。

Issue #4は開始時点でmain未取り込みのため、Issue #5の差分だけをレビューする場合は比較先を`feature/issue4`とします。Issue #4のmain取り込み後に比較先と差分を再確認してください。

レビューではREADMEどおりに起動・停止できること、127.0.0.1だけに公開すること、ビルド時にstatic/が含まれること、実行時にホストのRust環境・ソースマウントを必要としないことを確認してください。
