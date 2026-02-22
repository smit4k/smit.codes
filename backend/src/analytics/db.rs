use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use tracing::info;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    std::fs::create_dir_all("./db").expect("Failed to create db directory");

    let options =
        SqliteConnectOptions::from_str("sqlite:./db/analytics.db")?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    let schema = include_str!("../database/schema.sql");
    sqlx::raw_sql(schema).execute(&pool).await?;

    info!("Database schema initialized");

    Ok(pool)
}
