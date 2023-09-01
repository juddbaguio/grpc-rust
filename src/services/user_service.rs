use std::{fmt::Debug, sync::Arc};

use crate::repo::ports::UserRepoPort;

use crate::grpc::user::user_service_server::UserService;
use tonic::{Request, Response, Status};

use crate::grpc::user::{
    user_service_server::UserServiceServer, CreateUserPayload, CreateUserResponse,
};

#[derive(Debug, Clone)]
pub struct UserContext<T: UserRepoPort> {
    repo: Arc<T>,
}

#[tonic::async_trait]
impl<T: UserRepoPort> UserService for UserContext<T> {
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

pub fn new<T: UserRepoPort>(user_repo: T) -> UserServiceServer<UserContext<T>> {
    UserServiceServer::new(UserContext {
        repo: Arc::new(user_repo),
    })
}
