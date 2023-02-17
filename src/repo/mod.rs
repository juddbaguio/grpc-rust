use std::error::Error;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod movie_repo;
pub mod traits;
pub mod user_repo;

pub async fn connect_db() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://cinema-user:cinema-password@localhost:5432/cinema-db")
        .await?;
    Ok(db)
}
