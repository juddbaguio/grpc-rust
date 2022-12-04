use core::fmt::Debug;
use std::error::Error;

use async_trait::async_trait;

use crate::{user::{CreateUserResponse, CreateUserPayload}, movie::{MoviePayload, CreateMovieResponse}};

#[async_trait]
pub trait UserRepo: Debug {
    async fn create_user(&self, payload: CreateUserPayload) -> Result<CreateUserResponse, Box<dyn Error>>;
}

#[async_trait]
pub trait MovieRepo: Debug {
    async fn create_movie(&self, payload: MoviePayload) -> Result<CreateMovieResponse, Box<dyn Error>>;
}
