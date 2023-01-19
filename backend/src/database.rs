use sqlx::{Connection, Sqlite, SqliteConnection, SqlitePool};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::migrate::{MigrateDatabase, MigrateError};
use std::str::FromStr;


pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let database_url =
        dotenvy::var("DATABASE_URL").expect(".env does not contain `DATABASE_URL`");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

//
// Create database if it doesn't already exist!
//
