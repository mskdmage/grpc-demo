mod users;
mod sessions;
mod auth;
mod grpc;

use tokio::sync::Mutex;
use users::{Users, UsersInstance};
use sessions::{Sessions, SessionsInstance};
use auth::AuthService;
use grpc::auth::AuthServer;
use tonic::transport::Server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "[::0]:50051".parse()?;

    let users_service: Box<Mutex<dyn Users + Send + Sync + 'static>> = Box::new(
        Mutex::new(UsersInstance::new())
    );

    let sessions_service: Box<Mutex<dyn Sessions + Send + Sync + 'static>> = Box::new(
        Mutex::new(SessionsInstance::new())
    );

    let auth_service = AuthService::new(users_service, sessions_service);

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())

}

#[cfg(test)]
mod tests;