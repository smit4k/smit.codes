use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteSynchronous,
};
use std::{env, str::FromStr, time::Duration};
use tracing::info;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    std::fs::create_dir_all("./db").expect("Failed to create db directory");

    let db_url = env::var("ANALYTICS_DB_URL").unwrap_or_else(|_| "sqlite:./db/analytics.db".into());
    let max_connections = env::var("ANALYTICS_DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(10);

    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .min_connections(1)
        .connect_with(options)
        .await?;

    let schema = include_str!("../database/schema.sql");
    sqlx::raw_sql(schema).execute(&pool).await?;

    info!(
        "Database schema initialized at {} with max_connections={}",
        db_url, max_connections
    );

    Ok(pool)
}
