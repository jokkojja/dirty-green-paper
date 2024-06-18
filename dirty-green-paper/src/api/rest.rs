use axum::{routing::get, Router};
use std::env;
use tokio::net::TcpListener;

use crate::database;

use database::postgres::Database;

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

pub struct Api {
    pub listener: TcpListener,
    pub router: Router,
}

async fn hello_world() -> &'static str {
    return "Hello world";
}

impl Api {
    pub async fn from_env() -> Self {
        let config: ApiConfig = ApiConfig::from_env();

        let router: Router = Router::new()
            .route("/router1/hello", get(hello_world))
            .route("/router2/hello", get(|| async { "Hello world router 2" }))
            .route("/router3/hello", get(|| async { "Hello world router 3" }));

        let addr: String = config.host + ":" + &config.port;

        let listener: TcpListener = TcpListener::bind(addr).await.unwrap();

        Api { listener, router }
    }
}
