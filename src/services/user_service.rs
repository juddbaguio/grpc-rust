use std::{fmt::Debug, sync::Arc};

use crate::repo::traits::UserRepo;

use crate::grpc::user::user_service_server::UserService;
use tonic::{Request, Response, Status};

use crate::grpc::user::{
    user_service_server::UserServiceServer, CreateUserPayload, CreateUserResponse,
};

#[derive(Debug)]
pub struct UserContext {
    repo: Arc<dyn UserRepo + Send + Sync>,
}

#[tonic::async_trait]
impl UserService for UserContext {
    async fn create_user(
        &self,
        request: Request<CreateUserPayload>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let payload = request.get_ref().to_owned();
        let db = self.repo.clone();

        match db.create_user(payload).await {
            Ok(e) => Ok(Response::new(e)),
            Err(err) => {
                println!("{}", err);
                return Err(Status::aborted("wow"));
            }
        }
    }
}

pub fn new<T: UserRepo + Send + Sync + 'static>(user_repo: T) -> UserServiceServer<UserContext> {
    UserServiceServer::new(UserContext {
        repo: Arc::new(user_repo),
    })
}
