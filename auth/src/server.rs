mod api;
pub mod core;
pub mod structs;
pub mod traits;

use auth::{
    auth_server::{Auth, AuthServer},
    EmptyRequest, Response, SigninRequest, SignupRequest,
};
use tonic::{transport::Server, Request, Status};

pub mod auth {
    tonic::include_proto!("auth");
}

#[derive(Debug, Default)]
pub struct AuthService {}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn ping(&self, _: Request<EmptyRequest>) -> Result<tonic::Response<Response>, Status> {
        Ok(tonic::Response::new(Response {
            message: "pong".to_string(),
        }))
    }

    async fn signup(
        &self,
        req: Request<SignupRequest>,
    ) -> Result<tonic::Response<Response>, Status> {
        Ok(tonic::Response::new(Response {
            message: "signup request".to_string(),
        }))
    }

    async fn signin(
        &self,
        req: Request<SigninRequest>,
    ) -> Result<tonic::Response<Response>, Status> {
        Ok(tonic::Response::new(Response {
            message: "signup request".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:5000".parse().unwrap();
    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(address)
        .await?;
    println!("Running Auth Service on port 5000");
    Ok(())
}
