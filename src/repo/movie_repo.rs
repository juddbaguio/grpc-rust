use std::error::Error;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{movie::{MoviePayload, CreateMovieResponse}};

use super::traits::MovieRepo;


#[derive(Debug)]
pub struct MovieRepoContainer {
    db_conn: Pool<Postgres>
}

#[async_trait]
impl MovieRepo for MovieRepoContainer {
    async fn create_movie(&self, payload: MoviePayload) -> Result<CreateMovieResponse, Box<dyn Error>> {
        let conn = self.db_conn.clone();

        let res:(i64,) = sqlx::query_as(
            r#"
         INSERT INTO movies (title, director) 
         VALUES ($1,$2) 
         RETURNING id
        "#)
        .bind(payload.movie_title)
        .bind(payload.director)
        .fetch_one(&conn)
        .await?;

        
        println!("movie id - {} : CREATED", res.0);
        Ok(CreateMovieResponse { message: format!("movie id - {} : CREATED", res.0) })
    }
}

pub fn new(pg_conn: Pool<Postgres>) -> MovieRepoContainer {
    MovieRepoContainer {
        db_conn: pg_conn
    }
}
