# ライブラリ利用ルール

- 初回インストール時は `latest` を使用する。
- AI側で勝手にバージョン番号を判断して直打ちしない。
- インストール後、`package.json` と lock ファイルに記録されたバージョンを固定版として扱う。
- 実装中に勝手にライブラリを追加・更新しない。
- ライブラリ追加が必要な場合は、理由を説明してユーザ承認を得る。

## このプロジェクトへの適用

現在はRustプロジェクトであり、`package.json`はありません。外部ライブラリもまだ導入していません。管理対象に合わせて以下の方法で固定します。

| 対象 | 初回取得方法（承認後のみ） | 固定方法 |
| --- | --- | --- |
| Rustのライブラリ | `cargo add <crate名>`でバージョンを指定せず、Cargoに最新の互換性のある安定版を選択させる | 解決されたバージョンを確認し、直接依存は`Cargo.toml`で`=取得したバージョン`に固定。推移的依存を含め`Cargo.lock`を管理 |
| Rust / Cargo | `rustup toolchain install stable`で公式stableを取得 | `rustc +stable --version`の実測値を`rust-toolchain.toml`に記録。Cargoや標準ライブラリはこのツールチェーンに含まれる |
| RustのDockerイメージ | `docker pull rust:latest` | 取得後に`docker image inspect`で確認したdigest（`rust@sha256:...`）で固定 |
| JavaScriptのライブラリ（将来導入する場合） | `npm install <パッケージ名>@latest --save-exact` | `package.json`と`package-lock.json`を管理し、再取得時は`npm ci`を使用 |

Cargoとrustupはnpmの`latest`タグを使用しないため、上記の「バージョン番号を指定せず公式の最新安定版を取得する」操作を相当する手順とします。互換性制約などにより最新版を取得できない場合は、理由と実際に取得したバージョンを報告します。

通常の開発では`cargo run --locked`、`cargo test --locked`、`cargo clippy --locked -- -D warnings`を使用します。`--locked`はlockファイルの変更が必要な場合にエラーとし、意図しない依存バージョン変更を防ぎます。

追加・更新が必要な場合は、目的、対象、影響を説明し、ユーザー承認後に取得・固定・検証を行います。`cargo update`や`rustup update`、固定済みDockerイメージの差し替えも無断で実行しません。承認された導入・更新時のみlockファイルの生成・更新を許可し、結果は対象Issueの作業記録に残します。

GitやDocker Desktop / Engineはホストの開発ツールです。プロジェクトの依存ライブラリとは別に扱い、無断でインストール・更新しません。

## 参考

- [Cargoの依存バージョン指定](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [cargo add](https://doc.rust-lang.org/cargo/commands/cargo-add.html)
- [rust-toolchain.tomlによる固定](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)
