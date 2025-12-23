use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    std::fs::create_dir_all("./database").expect("Failed to create database directory");

    let options =
        SqliteConnectOptions::from_str("sqlite:./database/blog.db")?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    let schema = include_str!("../database/schema.sql");
    sqlx::raw_sql(schema).execute(&pool).await?;

    println!("✓ Database schema initialized");

    Ok(pool)
}
