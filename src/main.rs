mod repo;
mod services;

mod user {
    tonic::include_proto!("user");
}

mod movie {
    tonic::include_proto!("movie");
}

use std::error::Error;

use repo::{user_repo, movie_repo};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_conn = repo::connect_db().await?;

    let addr = "127.0.0.1:3000".parse().unwrap();
    let user_service = services::user_service::new(user_repo::new(db_conn.clone()));
    let movie_service = services::movie_service::new(movie_repo::new(db_conn.clone()));
    Server::builder()
        .add_service(user_service)
        .add_service(movie_service)
        .serve(addr)
        .await?;

    Ok(())
}
