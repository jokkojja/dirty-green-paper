use tokio::main;

mod api;

mod database;

mod builder;

mod models;

#[main]
async fn main() {
    builder::build_app().await;
}
