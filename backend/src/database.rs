// use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;


pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let database_url =
        dotenvy::var("DATABASE_URL").expect(".env does not contain `DATABASE_URL`");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
