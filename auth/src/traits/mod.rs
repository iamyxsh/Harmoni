use tonic::{async_trait, Status};

use crate::{
    auth::{Response, SigninRequest, SignupRequest},
    structs,
};

#[async_trait]
pub trait DBPort {
    async fn create_user(user: structs::user::User);
    async fn get_user(email: String);
}

#[async_trait]
pub trait CorePort {
    async fn signup(password: String) -> Result<String, String>;
    async fn signin(password: String, hash: String) -> Result<String, String>;
}

#[async_trait]
pub trait APITrait: Send + Sync {
    async fn signin(signin_req: SigninRequest) -> Result<tonic::Response<Response>, Status>;
    async fn signup(signup_req: SignupRequest) -> Result<tonic::Response<Response>, Status>;
}
