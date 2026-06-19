use axum::{Router, routing::get};

/// `GET /v1/systems/ping` のハンドラ。常に "pong" を返す。
async fn ping() -> &'static str {
    "pong"
}

#[tokio::main]
async fn main() {
    // パス階層をネストルーターで表現する。
    let systems = Router::new().route("/ping", get(ping));
    let v1 = Router::new().nest("/systems", systems);
    let app = Router::new().nest("/v1", v1);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
