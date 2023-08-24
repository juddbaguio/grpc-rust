use std::error::Error;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod movie_repo;
pub mod ports;
pub mod user_repo;

pub type DB = Pool<Postgres>;

pub async fn connect_db() -> Result<DB, Box<dyn Error>> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://grpc-user:grpc@localhost:5432/grpc-demo-db")
        .await?;
    Ok(db)
}
