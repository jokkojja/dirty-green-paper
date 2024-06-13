use axum::serve;

use tokio::main;

use dotenv::dotenv;

mod api;
use api::rest::Api;

#[main]
async fn main() {
    dotenv().ok();

    let api: Api = Api::from_env().await;

    serve(api.listener, api.router).await.unwrap();
}
