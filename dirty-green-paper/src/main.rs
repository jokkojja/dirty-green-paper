use axum::{routing::get, serve, Router};

use tokio::{main, net::TcpListener};

#[main]
async fn main() {
    let app: Router<_> = Router::new().route("/", get(|| async { "Hello world" }));

    let listener: TcpListener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    serve(listener, app).await.unwrap();
}
