use std::{fmt::Debug, sync::Arc};

use crate::grpc::movie::movie_service_server::MovieServiceServer;
use crate::repo::traits::MovieRepo;

use crate::grpc::movie::movie_service_server::MovieService;
use tonic::{Request, Response, Status};

use crate::grpc::movie::{CreateMovieResponse, MoviePayload};

#[derive(Debug)]
pub struct MovieContext {
    repo: Arc<dyn MovieRepo + Send + Sync>,
}

#[tonic::async_trait]
impl MovieService for MovieContext {
    async fn create_movie(
        &self,
        request: Request<MoviePayload>,
    ) -> Result<Response<CreateMovieResponse>, Status> {
        let payload = request.get_ref().to_owned();
        let db = self.repo.clone();

        match db.create_movie(payload).await {
            Ok(e) => Ok(Response::new(e)),
            Err(err) => {
                println!("{}", err);
                return Err(Status::aborted("wow"));
            }
        }
    }
}

pub fn new<T: MovieRepo + Send + Sync + 'static>(
    movie_repo: T,
) -> MovieServiceServer<MovieContext> {
    MovieServiceServer::new(MovieContext {
        repo: Arc::new(movie_repo),
    })
}
