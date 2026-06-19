use axum::{Router, routing::get};

async fn ping() -> &'static str {
    "pong"
}

#[tokio::main]
async fn main() {
    let systems = Router::new().route("/ping", get(ping));
    let v1 = Router::new().nest("/systems", systems);
    let app = Router::new().nest("/v1", v1);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
