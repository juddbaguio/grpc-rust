mod grpc;
mod repo;
mod services;

use std::error::Error;

use repo::{connect_db, movie_repo, user_repo};
use services::{movie_service, user_service};
use tonic::transport::Server;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let db_conn = connect_db().await?;

    let user_service = user_service::new(user_repo::new(db_conn.clone()));
    let movie_service = movie_service::new(movie_repo::new(db_conn.clone()));

    let addr = "127.0.0.1:3000".parse().unwrap();
    Server::builder()
        .add_service(user_service)
        .add_service(movie_service)
        .serve(addr)
        .await?;

    Ok(())
}
