use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

#[derive(Serialize)]
struct Metrics {
    cpu_percent: f64,
    memory_percent: f64,
}

fn app() -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/metrics", get(metrics))
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

async fn metrics() -> Json<Metrics> {
    // MVPでは実測値ではなく固定のダミー値を返す。
    Json(Metrics {
        cpu_percent: 42.5,
        memory_percent: 61.2,
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Dockerのポート公開でもアクセスできるよう、全IPv4インターフェースで待ち受ける。
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Monitoring API listening on http://localhost:8080");
    axum::serve(listener, app()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::time::Duration;

    // OSが割り当てる空きポートを使い、並列テストや起動済みアプリと競合させない。
    async fn request(method: &str, path: &str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, app()).await });
        let request = format!(
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
        );

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

        server.abort();
        let _ = server.await;
        response.unwrap()
    }

    fn assert_json_response(response: &str, expected_body: &str) {
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

    #[tokio::test]
    async fn health_returns_ok_json() {
        assert_json_response(&request("GET", "/api/health").await, r#"{"status":"ok"}"#);
    }

    #[tokio::test]
    async fn metrics_returns_dummy_percentages_json() {
        assert_json_response(
            &request("GET", "/api/metrics").await,
            r#"{"cpu_percent":42.5,"memory_percent":61.2}"#,
        );
    }

    #[tokio::test]
    async fn unknown_path_returns_not_found() {
        let response = request("GET", "/api/unknown").await;
        assert!(
            response.starts_with("HTTP/1.1 404 Not Found\r\n"),
            "{response}"
        );
    }

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
