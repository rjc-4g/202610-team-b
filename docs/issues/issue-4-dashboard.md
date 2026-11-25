# Issue #4：サーバー監視ダッシュボードを実装

- 対象：[Issue #4](https://github.com/rjc-4g/202610-team-b/issues/4)
- 記録日：2026-11-25
- 作業ブランチ：`feature/issue4`

## 参照した資料

- [開発ルール](../development-rules.md)、[ライブラリ利用ルール](../dependencies.md)、[README](../../README.md)
- [Issue #2の作業記録](issue-2-setup.md)、[Issue #3の作業記録](issue-3-monitoring-api.md)
- [実装ガイド](../project-inception/02-implementation.md)、[決定事項](../project-inception/06-decisions-and-open-questions.md)、[UI参考資料と画像利用方針](../project-inception/07-design-references.md)

## 実装と判断

1. 既存の`feature/issue4`ブランチと、作業開始時の変更がないことを確認しました。既存のダミー監視APIをそのまま利用しています。
2. `static/index.html`、`static/style.css`、`static/app.js`を追加しました。CPU・メモリ使用率、接続状態、最終更新日時を表示し、狭い画面ではカードを縦に並べます。ダミーデータである旨を画面に明記しています。
3. `/`と`/index.html`で画面、`/style.css`と`/app.js`で参照資産を配信します。HTML・CSS・JavaScriptそれぞれのContent-Typeを設定しています。未定義パスの404、APIのJSON契約を維持しています。
4. `src/constants.rs`に待ち受けアドレスと配信資産を集約しました。`include_str!`によるビルド時の埋め込みを採用しています。企画時の`ServeDir`例とは異なりますが、固定の3ファイルというMVPの範囲では追加ライブラリが不要で、実行時の作業フォルダに依存せず配信できます。
5. ブラウザから相対URL `/api/metrics`を`fetch`し、初回は即時、その後は5秒ごとに更新します。キャッシュを使わず、通信中の重複取得を防ぎます。応答待ちは4秒で打ち切り、失敗後も自動で再接続します。
6. HTTPエラー、通信失敗、不正JSON、使用率の型・範囲の異常をエラーとして扱います。成功時だけ両指標と日時を更新します。失敗時は前回取得値であることを明記し、初回失敗では未取得表示を保ちます。接続復帰時はエラー表示を解除します。
7. 外部画像・外部フォント・JavaScriptライブラリは使用していません。任意の時系列グラフと実測値取得は対象外です。使用率バーは現在値を示すだけで、履歴グラフではありません。
8. 初級者向けに日本語コメントを追加し、READMEに起動方法、更新仕様、単体テストと手動確認手順を記載しました。

## 依存とIssue #5への引き継ぎ

- ライブラリの追加・更新はありません。`Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml`と既存Dockerイメージのdigestは変更していません。
- 待ち受けは既存どおり`0.0.0.0:8080`です。画面とAPIは同じサーバーから配信します。
- Dockerビルド時は`src/`だけでなく`static/`も必要です。実行用イメージへの`static/`の別途コピーは不要です。
- HTML・CSS・JavaScriptの変更後は再ビルド・再起動が必要です。実行時にファイルを差し替える方式ではありません。
- Dockerfile・Composeの作成とその検証はIssue #5の対象です。今回は既存READMEの`docker run`方式を使用しました。

## 検証結果（2026-11-25）

Rustは既存の固定済みDockerイメージ（`rust@sha256:7e9eb23ac694dda816c76933ab63b2703e6becbf3b10da90970124656788d47b`）と`rust-toolchain.toml`の固定版で検証しました。JavaScriptの単体テストには導入済みのNode.js v24.6.0を使用しました。Node.jsは画面の利用・Rustアプリの実行には不要です。

| 確認項目 | 結果 |
| --- | --- |
| `cargo run --locked` | 起動成功。読み取り専用ソースマウント、`127.0.0.1:8080:8080`の公開で画面とAPIを確認 |
| `cargo test --locked` | 6件成功。既存API、画面・資産の200とContent-Type、404、公開パスのPOSTに対する405 |
| `cargo fmt --check` | `cargo fmt`で整形後、成功 |
| `cargo clippy --locked --all-targets -- -D warnings` | テストコードを含め成功 |
| `node --check static/app.js` | 成功 |
| `node --test tests/dashboard.test.cjs` | 6件成功。初回・定期更新、切断と復帰、HTTPエラー、不正応答、タイムアウト・重複防止、境界値 |
| 実API | HTTP 200、application/json、CPU 42.5・メモリ61.2を確認 |
| Chromeによる表示確認 | 1280px幅と390px幅でスクリーンショットを確認。CPU 42.5%、メモリ61.2%、接続済み、日時、ダミー表示を確認 |
| Chromeによる自動更新 | 実リクエスト間隔約5.013秒、最終更新日時の変化を確認 |
| Chromeによる通信切断・復帰 | ブラウザのネットワークを一時的にオフラインにし、エラー表示と前回値・日時の保持を確認。オンライン復帰後、再読み込みなしで接続済みに戻ることを確認 |
| 狭い画面 | 390pxのビューポートでカードが縦に並び、横方向のはみ出しがないことを確認 |
| `git diff --check` | 成功 |

JavaScriptの単体テストはNode.js標準機能でDOM・通信・タイマーを代替し、実際の`static/app.js`を実行します。ブラウザでの動作確認とは分けて記録しています。

## 環境上の問題と対応

- 通常のコマンド実行とブラウザ連携ツールで`CreateProcessSecurityEnvironment failed (HRESULT = 0x8007000F)`が発生しました。コマンドは許可されたサンドボックス外実行で実施しました。
- Dockerエンジンが停止していたため、既存Docker Desktopを起動して検証しました。Docker自体のインストール・更新は行っていません。
- ブラウザ連携ツールの代わりに、既存Chromeを一時プロファイルのヘッドレスモードで起動し、表示・更新・ネットワーク切断と復帰を確認しました。ブラウザやテスト用ライブラリの追加導入はありません。
- WindowsネイティブでのRustビルド、Chrome以外のブラウザ、実機スマートフォンでの操作は未確認です。
- この作業用の検証コンテナは停止・削除済みです。ヘッドレスChromeも終了しています。

## PR・レビュー

開発ルールに従い、ローカルコミットまでを実施します。push、PR作成、GitHub上のIssue・Project更新は行いません。管理・レビュー担当者の確認は未実施です。

レビューでは次の点を確認してください。

- READMEの手順で画面を開け、5秒ごとに日時が更新されること。
- ダミー値であることと、通信失敗時の前回値であることが明確であること。
- 通信復帰後に自動更新が再開すること。
- 既存APIの仕様と404・405応答を維持していること。
- 埋め込み配信の再ビルド要件とIssue #5のビルド対象に`static/`を含めることが共有されていること。
