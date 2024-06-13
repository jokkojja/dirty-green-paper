use dotenv::dotenv;
use std::env;
use tokio::spawn;
use tokio_postgres::{Client, NoTls};

use crate::models::money;

pub struct Database {
    client: Client,
}

impl Database {
    async fn from_env() -> Self {
        dotenv().ok();
        let database_host = env::var("DATABASE_HOST").unwrap();

        let (client, connection) = tokio_postgres::connect(&database_host, NoTls)
            .await
            .unwrap();

        spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        return Self { client };
    }

    async fn add_income(income: money::Income) {}
    async fn add_extense(income: money::Expense) {}
    async fn remove_extense() {}
    async fn remove_income() {}
}
