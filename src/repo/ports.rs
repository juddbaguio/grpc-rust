use core::fmt::Debug;
use std::error::Error;

use async_trait::async_trait;

use crate::grpc::{
    movie::{CreateMovieResponse, MoviePayload},
    user::{CreateUserPayload, CreateUserResponse},
};

use super::{movie_repo::MovieRepoContainer, user_repo::UserRepoContainer};

#[async_trait]
pub trait UserRepoPort: Debug + Send + Sync {
    async fn create_user(
        &self,
        payload: CreateUserPayload,
    ) -> Result<CreateUserResponse, Box<dyn Error>>;
}

#[async_trait]
pub trait MovieRepoPort: Debug + Send + Sync {
    async fn create_movie(
        &self,
        payload: MoviePayload,
    ) -> Result<CreateMovieResponse, Box<dyn Error>>;
}

#[derive(Debug)]
pub enum MovieRepo {
    Concrete(MovieRepoContainer),
}

#[derive(Debug)]
pub enum UserRepo {
    Concrete(UserRepoContainer),
}

#[async_trait]
impl MovieRepoPort for MovieRepo {
    async fn create_movie(
        &self,
        payload: MoviePayload,
    ) -> Result<CreateMovieResponse, Box<dyn Error>> {
        match *self {
            Self::Concrete(ref container) => container.create_movie(payload).await,
        }
    }
}

#[async_trait]
impl UserRepoPort for UserRepo {
    async fn create_user(
        &self,
        payload: CreateUserPayload,
    ) -> Result<CreateUserResponse, Box<dyn Error>> {
        match *self {
            Self::Concrete(ref container) => container.create_user(payload).await,
        }
    }
}
