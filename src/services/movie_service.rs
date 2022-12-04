use std::{fmt::Debug};

use crate::movie::movie_service_server::MovieServiceServer;
use crate::repo::traits::MovieRepo;

use tonic::{Request, Response, Status};
use crate::movie::movie_service_server::MovieService;

use crate::movie::{MoviePayload, CreateMovieResponse};


#[derive(Debug)]
pub struct MovieContext {
    repo: Box<dyn MovieRepo + Send + Sync + 'static>,
}

#[tonic::async_trait]
impl MovieService for MovieContext {
    async fn create_movie(
        &self,
        request: Request<MoviePayload>,
    ) -> Result<Response<CreateMovieResponse>, Status> {
        let payload = request.get_ref().to_owned();

        match self.repo.create_movie(payload).await {
            Ok(e) => Ok(Response::new(e)),
            Err(err) => {
                println!("{}", err);
                return Err(Status::aborted("wow"))
            }
        }        
    }
}

pub fn new<T: MovieRepo + Send + Sync + 'static>(movie_repo: T) -> MovieServiceServer<MovieContext> {
    let data = MovieServiceServer::new(MovieContext {
        repo: Box::new(movie_repo),
    });

    return data;
}