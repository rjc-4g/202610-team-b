# Issue #3：Axumでダミー監視APIを実装

- 対象：[Issue #3](https://github.com/rjc-4g/202610-team-b/issues/3)
- 記録日：2026-10-21
- 作業ブランチ：`feature/issue3`

## 実装と判断

- `GET /api/health`は`{"status":"ok"}`、`GET /api/metrics`は`{"cpu_percent":42.5,"memory_percent":61.2}`を返します。両方ともHTTP 200、`Content-Type: application/json`です。
- Serdeの`Serialize`とAxumの`Json`で応答を生成します。監視値は固定のダミー値で、実測値取得や監視画面は対象外です。
- `0.0.0.0:8080`で待ち受け、`localhost:8080`からアクセスできます。Dockerのポート公開にも対応するため全IPv4インターフェースにbindします。READMEのDocker手順ではホストの`127.0.0.1`にだけ公開します。
- ルーター生成を`app()`に分離し、本番起動とHTTPテストで共有します。起動時のbind失敗などのI/Oエラーは呼び出し元へ返します。
- READMEをAPIの起動・停止・curl確認・テスト手順に更新しました。

## 依存の承認と固定

ユーザーから「3ライブラリの導入・固定を承認する」と明示的な承認を受け、以下をバージョン未指定で実行しました。

```sh
cargo add axum
cargo add tokio --features macros,rt-multi-thread,net
cargo add serde --features derive
```

取得結果を確認し、直接依存を`Cargo.toml`で完全一致指定に変更しました。推移的依存も`Cargo.lock`に記録しています。

| ライブラリ | 実際に取得・固定したバージョン | 用途 |
| --- | --- | --- |
| Axum | `=0.8.9` | HTTPルーティングとJSON応答 |
| Tokio | `=1.54.0` | 非同期ランタイム、マクロ、TCPリスナー |
| Serde | `=1.0.229` | JSON応答用構造体のシリアライズ |

テスト専用ライブラリは追加せず、HTTPクライアント部分はRust標準ライブラリを使用しています。既存のRust 1.99.1とDockerイメージdigestは変更していません。

## 検証結果（2026-10-21）

既存の固定済みRust Dockerイメージを使用しました。コンテナ内の実測値は`rustc 1.99.1`、`cargo 1.99.1`です。ホストWindowsのRustはPATH上にないため、Windowsネイティブビルドは未確認です。

| 確認項目 | 結果 |
| --- | --- |
| `cargo fmt --check` | 成功（`cargo fmt`で整形後） |
| `cargo test --locked` | 成功、4件 |
| `cargo clippy --locked -- -D warnings` | 成功 |
| `cargo clippy --locked --all-targets -- -D warnings` | テストコードを含め成功 |
| READMEの読み取り専用Dockerマウントでの検証 | クリーンなコンテナで依存取得・テスト・fmt・clippyが成功 |
| `git diff --check` | 成功 |
| `cargo run --locked` | 起動成功、ポート8080で待ち受け |
| `curl.exe --fail-with-body --include --max-time 10 http://localhost:8080/api/health` | HTTP 200、application/json、`{"status":"ok"}` |
| `curl.exe --fail-with-body --include --max-time 10 http://localhost:8080/api/metrics` | HTTP 200、application/json、`{"cpu_percent":42.5,"memory_percent":61.2}` |

自動テストは実際のTCP接続で両APIの200・Content-Type・JSON本文、未定義パスの404、両APIへのPOSTの405を確認します。ポートはOSの自動割り当てを使い、接続・読み書きに5秒のタイムアウトを設定しています。

サンドボックス内からはDocker APIとホストのlocalhostへ接続できなかったため、許可されたサンドボックス外実行で検証しました。

## PR・レビュー

開始時点で、前提となるIssue #2の[PR #9](https://github.com/rjc-4g/202610-team-b/pull/9)は未マージでした。Issue #3の差分だけを確認できるよう、PRの比較先は`feature/issue2`とする方針です。PR #9のマージ後、比較先を`main`に変更し、差分とCIを再確認します。

PR作成と担当者によるPRレビューは現時点で未実施です。レビュー完了はIssueの完了条件なので、実装・動作検証のみでIssue全体を完了扱いにしません。

レビューではAPIのJSON契約、ダミー値の明示、依存の完全一致固定、READMEの起動・停止手順を確認してください。

## 実装時の参照

- [Axum公式ドキュメント](https://docs.rs/axum/latest/axum/)
- [Tokio TcpListener](https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html)
- [Serde derive](https://serde.rs/derive.html)
