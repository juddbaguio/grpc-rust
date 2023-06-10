use std::error::Error;

use sqlx::{postgres::PgPoolOptions, Executor, Pool, Postgres, Transaction};

pub mod movie_repo;
pub mod traits;
pub mod user_repo;

pub type DB = Pool<Postgres>;
pub trait TX<'c>: Executor<'c, Database = Postgres> {}
impl<'c> TX<'c> for &Pool<Postgres> {}
impl<'c> TX<'c> for &'c mut Transaction<'_, Postgres> {}

pub async fn connect_db() -> Result<DB, Box<dyn Error>> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://cinema-user:cinema-password@localhost:5432/cinema-db")
        .await?;
    Ok(db)
}
