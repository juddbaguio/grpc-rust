use std::{fmt::Debug};

use crate::repo::{traits::UserRepo};

use tonic::{Request, Response, Status};
use crate::user::user_service_server::UserService;

use crate::user::{user_service_server::UserServiceServer, CreateUserPayload, CreateUserResponse};


#[derive(Debug)]
pub struct UserContext {
    repo: Box<dyn UserRepo + Send + Sync + 'static>,
}

#[tonic::async_trait]
impl UserService for UserContext {
    async fn create_user(
        &self,
        request: Request<CreateUserPayload>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let payload = request.get_ref().to_owned();

        match self.repo.create_user(payload).await {
            Ok(e) => Ok(Response::new(e)),
            Err(err) => {
                println!("{}", err);
                return Err(Status::aborted("wow"))
            }
        }        
    }
}

pub fn new<T: UserRepo + Send + Sync + 'static>(user_repo: T) -> UserServiceServer<UserContext> {
    let data = UserServiceServer::new(UserContext {
        repo: Box::new(user_repo),
    });

    return data;
}