use crate::api::rest::Api;
use crate::database;
use crate::database::postgres::Database;
use axum::serve;
use dotenv::dotenv;

pub async fn build_app() {
    dotenv().ok();

    let api: Api = Api::from_env().await;

    // TODO: Research aboud DI in rust and axum
    let database: Database = Database::from_env().await;

    serve(api.listener, api.router).await.unwrap();
}
