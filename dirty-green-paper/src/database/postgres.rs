use dotenv::dotenv;
use std::env;
use tokio::spawn;
use tokio_postgres::{Client, NoTls};

use models::money;

struct DataBase {
    client: Client,
}

impl DataBase {
    async fn from_env() -> Self {
        dotenv().ok();
        let database_host = env::var("DATABASE_HOST").unwrap();

        let (client, connection) = tokio_postgres::connect(&database_host, NoTls).await?;

        spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        return Self { client };
    }

    async fn add_income(income: Income) {}
    async fn add_extense(income: Extens) {}
    async fn remove_extense() {}
    async fn remove_income() {}
}
