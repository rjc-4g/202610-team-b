# サーバー監視ダッシュボード

Rust + Axum + HTML/JavaScript + Docker Composeで開発するサーバー監視ダッシュボードです。
まずはダミー値でCPU・メモリ使用率を表示するMVPを完成させ、コンテナの実測値取得は追加チャレンジとします。

## フォルダ構成

```text
.
├── .github/
│   └── workflows/  # GitHub ActionsのCI設定
├── docs/           # 設計・作業手順・レビュー・成果発表のドキュメント
├── src/            # Rustのソースコード
├── static/         # HTML・CSS・JavaScript
├── .gitignore
└── README.md
```

空フォルダをGitで管理するため、`.gitkeep`を配置しています。実ファイルを追加したら削除できます。

## 開発の進め方

1. 対応する[Issue](https://github.com/rjc-4g/202610-team-b/issues)を確認する。
2. featureブランチで作業する。
3. Pull Requestを作成し、レビューを受ける。

現在はフォルダ構成のみを準備した状態です。Rustプロジェクトの初期化はSetup Issueで行い、実装に合わせて起動方法・API仕様・制約を追記します。

ドキュメントの格納方針は[docs/README.md](docs/README.md)を参照してください。
