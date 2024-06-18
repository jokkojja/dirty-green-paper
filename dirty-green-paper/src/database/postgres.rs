use crate::models::money;
use dotenv::dotenv;
use std::env;
use tokio::spawn;
use tokio_postgres::{Client, Error, NoTls};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn from_env() -> Self {
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
