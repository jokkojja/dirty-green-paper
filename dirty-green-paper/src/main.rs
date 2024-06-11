use axum::{routing::get, serve, Router};

use tokio::{main, net::TcpListener};

use std::env;

use dotenv::dotenv;

struct ApiConfig {
    host: String,
    port: String,
}

impl ApiConfig {
    fn from_env() -> Self {
        let host: String = env::var("HOST").unwrap();
        let port: String = env::var("PORT").unwrap();

        return ApiConfig {
            host: host,
            port: port,
        };
    }
}

struct Api {
    listener: TcpListener,
    router: Router,
}

impl Api {
    async fn from_env() -> Self {
        let config: ApiConfig = ApiConfig::from_env();

        let router: Router = Router::new()
            .route("/router1/hello", get(|| async { "Hello world router 1" }))
            .route("/router2/hello", get(|| async { "Hello world router 2" }))
            .route("/router3/hello", get(|| async { "Hello world router 3" }));

        let addr: String = config.host + ":" + &config.port;

        let listener: TcpListener = TcpListener::bind(addr).await.unwrap();

        Api { listener, router }
    }
}

#[main]
async fn main() {
    dotenv().ok();

    let api: Api = Api::from_env().await;

    serve(api.listener, api.router).await.unwrap();
}
