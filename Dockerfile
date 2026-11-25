# Issue #2で取得・固定したRust公式イメージを再利用する。
# ビルド用のコンパイラやCargoキャッシュは、このステージだけに保持する。
FROM rust@sha256:7e9eb23ac694dda816c76933ab63b2703e6becbf3b10da90970124656788d47b AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY src/ ./src/
# include_str!で画面を埋め込むため、static/はビルド時に必要。
COPY static/ ./static/
RUN cargo build --release --locked

# 実行に必要な共有ライブラリとローダーを同じイメージから取り出す。
# lddの出力には「lib名 => /絶対パス」と「/ローダーの絶対パス」の2形式がある。
# awkで絶対パスを抽出し、cp --parentsで元の配置を保ってコピーする。
# CPUアーキテクチャ固有のライブラリパスをDockerfileに直書きしない。
RUN set -eu; \
    mkdir -p /runtime; \
    ldd target/release/server-monitoring-dashboard > /tmp/runtime-libraries.txt; \
    awk '$2 == "=>" { print $3 } $1 ~ /^\// { print $1 }' /tmp/runtime-libraries.txt > /tmp/runtime-library-paths.txt; \
    while IFS= read -r library; do \
        cp --parents "$library" /runtime; \
    done < /tmp/runtime-library-paths.txt

# scratchは空のイメージ。別のOSイメージや追加パッケージを取得せず、
# 実行ファイルと必要な共有ライブラリだけで動かす（シェル・Cargoは含まない）。
FROM scratch AS runtime
COPY --from=builder /runtime/ /
COPY --from=builder /app/target/release/server-monitoring-dashboard /app/server-monitoring-dashboard
WORKDIR /app
# ユーザー名の登録は不要。root権限を持たない数値UID/GIDで実行する。
USER 65532:65532
EXPOSE 8080
# exec形式によりアプリが直接起動し、Dockerの停止シグナルを受け取る。
ENTRYPOINT ["/app/server-monitoring-dashboard"]
