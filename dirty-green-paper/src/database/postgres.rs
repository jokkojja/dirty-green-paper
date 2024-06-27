use crate::models::money;
use dotenv::dotenv;
use std::env;
use tokio::spawn;
use tokio_postgres::{Client, Config, Error, NoTls};

pub struct Database {
    client: Client,
}

fn prepare_config() -> Config {
    let mut new_config = Config::new();

    new_config.user("test");
    new_config.dbname("test");
    new_config.password("test");
    new_config.port(5432);
    new_config.host("postgres");

    new_config
}

impl Database {
    pub async fn from_env() -> Self {
        dotenv().ok();
        let database_config = prepare_config();

        let (client, connection) = database_config.connect(NoTls).await.unwrap();

        spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        return Self { client };
    }

    pub async fn add_income(&self, income: money::Income) -> Result<(), Error> {
        self.client
            .execute(
                "INSERT INTO incomes (amount, currency, category, description, date) VALUES ($1, $2, $3, $4, $5)",
                &[
                    &income.amount,
                    &income.currency.as_str(),
                    &income.category.as_str(),
                    &income.description.as_str(),
                    &income.date,
                ],
            )
            .await?;
        Ok(())
    }
    pub async fn add_expense(&self, income: money::Expense) -> Result<(), Error> {
        self.client
            .execute(
                "INSERT INTO expense (amount, currency, category, description, date) VALUES ($1, $2, $3, $4, $5)",
                &[
                    &income.amount,
                    &income.currency.as_str(),
                    &income.category.as_str(),
                    &income.description.as_str(),
                    &income.date,
                ],
            )
            .await?;
        Ok(())
    }
    async fn remove_extense() {}
    async fn remove_income() {}
}
