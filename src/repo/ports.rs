use core::fmt::Debug;
use std::error::Error;

use async_trait::async_trait;

use crate::grpc::{
    movie::{CreateMovieResponse, MoviePayload},
    user::{CreateUserPayload, CreateUserResponse},
};

#[async_trait]
pub trait UserRepoPort: Debug + Send + Sync + 'static {
    async fn create_user(
        &self,
        payload: CreateUserPayload,
    ) -> Result<CreateUserResponse, Box<dyn Error>>;
}

#[async_trait]
pub trait MovieRepoPort: Debug + Send + Sync + 'static {
    async fn create_movie(
        &self,
        payload: MoviePayload,
    ) -> Result<CreateMovieResponse, Box<dyn Error>>;
}
