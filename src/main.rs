use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;

// ヘルスチェックの応答データ。SerializeはJSONに変換するための処理を自動生成する。
// &'static strは、プログラムの実行中ずっと有効な文字列への参照を表す。
#[derive(Serialize)]
struct Health {
    status: &'static str,
}

// CPU・メモリ使用率の応答データ。f64は小数を扱う型で、値の単位はパーセント。
#[derive(Serialize)]
struct Metrics {
    cpu_percent: f64,
    memory_percent: f64,
}

/// HTTPメソッド・URLのパスと、呼び出す関数の対応を登録したRouterを返す。
/// 引数は不要で、本番のサーバーとテストの両方から利用する。
/// get(health)は「GETでアクセスされたらhealth関数を呼ぶ」という登録であり、
/// この関数の中でhealthを実行するわけではない。
fn app() -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/metrics", get(metrics))
}

/// GET /api/healthを受け付け、サーバーが応答できることを示すデータを返す。
/// async fnは非同期関数を定義する構文で、ここではAxumのハンドラーとして使う。
/// 引数は不要。戻り値の`Json<Health>`をAxumがJSON本文とHTTP応答に変換する。
async fn health() -> Json<Health> {
    // 最後の式にセミコロンを付けないことで、その値が関数の戻り値になる。
    Json(Health { status: "ok" })
}

/// GET /api/metricsを受け付け、CPU・メモリ使用率をJSON応答用のデータとして返す。
/// 引数は不要で、戻り値は`Json<Metrics>`。現在は固定のダミー値を使用する。
/// 実測値の取得を導入するときは、このデータを組み立てる部分を取得処理に置き換える。
async fn metrics() -> Json<Metrics> {
    // MVPでは実測値ではなく固定のダミー値を返す。
    Json(Metrics {
        cpu_percent: 42.5,
        memory_percent: 61.2,
    })
}

/// アプリの起動時に実行され、ポート8080でHTTPリクエストを待ち受ける。
/// #[tokio::main]は、非同期処理を動かす実行環境を準備するマクロ。
/// 引数は不要。`std::io::Result<()>`は、成功時に値を持たず、失敗時にI/Oエラーを返す型。
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Dockerのポート公開でもアクセスできるよう、全IPv4インターフェースで待ち受ける。
    // .awaitは非同期処理の完了を待つ。末尾の?は失敗した場合にmainからエラーを返す。
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Monitoring API listening on http://localhost:8080");
    // app()のルーティングに従ってリクエストを処理し、サーバーの実行結果を返す。
    axum::serve(listener, app()).await
}

// このモジュールはcargo testでのビルド時だけ有効になり、通常の実行には含まれない。
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::time::Duration;

    /// テスト用サーバーへHTTPリクエストを1回送り、ステータス行・ヘッダー・本文を返す。
    /// methodは"GET"などのメソッド、pathは"/api/health"などのパスを指定する。
    /// &strは文字列を借りて参照する型で、Stringはこの関数が生成して返す文字列の型。
    /// サーバーは通信完了後に停止する。unwrap()は失敗時にテストを失敗させるために使う。
    async fn request(method: &str, path: &str) -> String {
        // ポート0を指定するとOSが空きポートを割り当て、他のテストやアプリと競合しない。
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        // spawnでサーバーを別の非同期タスクとして動かす。
        // moveは、listenerの所有権をこのタスクへ移し、タスク内で使えるようにする。
        let server = tokio::spawn(async move { axum::serve(listener, app()).await });
        let request = format!(
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
        );

        // 標準ライブラリの同期通信は待機中にスレッドを止めるため、専用のスレッドで行う。
        // 同じ非同期実行スレッドで通信を待つと、サーバー側の処理が進まなくなる恐れがある。
        let response = tokio::task::spawn_blocking(move || {
            let timeout = Duration::from_secs(5);
            let mut stream = TcpStream::connect_timeout(&address, timeout).unwrap();
            stream.set_read_timeout(Some(timeout)).unwrap();
            stream.set_write_timeout(Some(timeout)).unwrap();
            stream.write_all(request.as_bytes()).unwrap();
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            response
        })
        .await;

        // 常時待ち受けるテスト用サーバーを中止し、終了を待ってから応答を返す。
        // let _は、中止に伴うタスクの終了結果を意図的に使わないことを表す。
        server.abort();
        let _ = server.await;
        response.unwrap()
    }

    /// 受信したHTTP応答が「200 OK・JSON形式・指定した本文」であることを検証する。
    /// responseはHTTP応答全体、expected_bodyは期待するJSON文字列を、参照として受け取る。
    /// 戻り値はなく、条件を満たさなければassert!またはassert_eq!でテストを失敗させる。
    fn assert_json_response(response: &str, expected_body: &str) {
        // HTTPでは空行（\r\n\r\n）がヘッダーと本文の区切りになる。
        let (headers, body) = response.split_once("\r\n\r\n").unwrap();
        assert!(headers.starts_with("HTTP/1.1 200 OK\r\n"), "{headers}");
        assert!(
            headers
                .lines()
                .any(|line| line.eq_ignore_ascii_case("content-type: application/json")),
            "{headers}"
        );
        assert_eq!(body, expected_body);
    }

    /// ヘルスチェックがHTTP 200と{"status":"ok"}のJSONを返すことを検証する。
    /// #[tokio::test]は非同期テスト用の実行環境を準備する。引数・戻り値は不要。
    #[tokio::test]
    async fn health_returns_ok_json() {
        // r#"..."#は、JSON中の引用符をエスケープせずに書ける文字列の記法。
        assert_json_response(&request("GET", "/api/health").await, r#"{"status":"ok"}"#);
    }

    /// 監視APIがHTTP 200と、仕様どおりのダミー使用率をJSONで返すことを検証する。
    /// 引数・戻り値は不要。期待値を明記して、返却値の誤変更を検出する。
    #[tokio::test]
    async fn metrics_returns_dummy_percentages_json() {
        assert_json_response(
            &request("GET", "/api/metrics").await,
            r#"{"cpu_percent":42.5,"memory_percent":61.2}"#,
        );
    }

    /// 未登録のURLへGETすると404 Not Foundになることを検証する。
    /// 引数・戻り値は不要。成功用のAPIに誤って振り分けられないことも確認する。
    #[tokio::test]
    async fn unknown_path_returns_not_found() {
        let response = request("GET", "/api/unknown").await;
        assert!(
            response.starts_with("HTTP/1.1 404 Not Found\r\n"),
            "{response}"
        );
    }

    /// 両APIへPOSTすると405 Method Not Allowedになることを検証する。
    /// 引数・戻り値は不要。forで両方のパスを順に確認し、読み取り用のAPIを守る。
    #[tokio::test]
    async fn post_is_not_allowed_on_read_only_endpoints() {
        for path in ["/api/health", "/api/metrics"] {
            let response = request("POST", path).await;
            assert!(
                response.starts_with("HTTP/1.1 405 Method Not Allowed\r\n"),
                "{response}"
            );
        }
    }
}
