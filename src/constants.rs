//! アプリの設定と配信資産を集約する。変更時は再ビルドして反映する。

/// Dockerでも接続できるよう全IPv4インターフェースで待ち受けるアドレスとポート。
pub const SERVER_ADDRESS: &str = "0.0.0.0:8080";

// include_str!はビルド時にUTF-8ファイルを読み込み、実行ファイルへ埋め込むマクロ。
// 実行時の作業フォルダに依存せず、追加ライブラリなしで同じ画面を配信できる。
pub const INDEX_HTML: &str = include_str!("../static/index.html");
pub const STYLE_CSS: &str = include_str!("../static/style.css");
pub const APP_JS: &str = include_str!("../static/app.js");
