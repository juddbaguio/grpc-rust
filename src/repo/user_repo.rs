use std::error::Error;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::grpc::user::{CreateUserPayload, CreateUserResponse};

use super::ports::UserRepoPort;

#[derive(Debug)]
pub struct UserRepoContainer {
    db_conn: Pool<Postgres>,
}

#[async_trait]
impl UserRepoPort for UserRepoContainer {
    async fn create_user(
        &self,
        payload: CreateUserPayload,
    ) -> Result<CreateUserResponse, Box<dyn Error>> {
        let res: (i32,) = sqlx::query_as(
            r#"
         INSERT INTO users (first_name, last_name, age) 
         VALUES ($1,$2,$3) 
         RETURNING id
        "#,
        )
        .bind(payload.first_name)
        .bind(payload.last_name)
        .bind(payload.age)
        .fetch_one(&self.db_conn)
        .await?;

        println!("user id - {} : CREATED", res.0);
        Ok(CreateUserResponse {
            message: format!("user id - {} : CREATED", res.0),
        })
    }
}

pub fn new(pg_conn: Pool<Postgres>) -> UserRepoContainer {
    UserRepoContainer { db_conn: pg_conn }
}
