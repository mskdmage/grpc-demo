use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use crate::users::Users;
use crate::sessions::Sessions;
use crate::grpc::auth::{
    Auth,
    SignInRequest,
    SignInResponse,
    SignOutRequest,
    SignOutResponse,
    SignUpRequest,
    SignUpResponse,
    StatusCode,
};

pub struct AuthService {
    users_service: Box<Mutex<dyn Users + Send + Sync>>,
    sessions_service: Box<Mutex<dyn Sessions + Send + Sync>>,
}

impl AuthService {
    pub fn new(
        users_service: Box<Mutex<dyn Users + Send + Sync>>,
        sessions_service: Box<Mutex<dyn Sessions + Send + Sync>>
    ) -> Self {
        Self {
            users_service,
            sessions_service,
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_up(
        &self,
        request: Request<SignUpRequest>
    ) -> Result<Response<SignUpResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let mut users = self.users_service.lock().await;
        let create_result = users.create_user(req.username.clone(), req.password.clone());

        let status_code = match create_result {
            Ok(_) => StatusCode::Success as i32,
            Err(_) => StatusCode::Failure as i32,
        };

        let reply = SignUpResponse {
            status_code,
        };

        Ok(Response::new(reply))
    }

    async fn sign_in(
        &self,
        request: Request<SignInRequest>
    ) -> Result<Response<SignInResponse>, Status> {

        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let user_uuid = {
            let users = self.users_service.lock().await;
            users.get_user_uuid(req.username.clone(), req.password.clone())
        };

        match user_uuid {
            Some(uuid) => {
                let session_token = {
                    let mut sessions = self.sessions_service.lock().await;
                    sessions.create_session(&uuid)
                };

                let reply = SignInResponse {
                    status_code: StatusCode::Success as i32,
                    user_uuid: uuid,
                    session_token,
                };
                Ok(Response::new(reply))
            }
            None => {
                let reply = SignInResponse {
                    status_code: StatusCode::Failure as i32,
                    user_uuid: "".to_string(),
                    session_token: "".to_string(),
                };
                Ok(Response::new(reply))
            }
        }
    }

    async fn sign_out(
        &self,
        request: Request<SignOutRequest>
    ) -> Result<Response<SignOutResponse>, Status> {

        println!("Got a request: {:?}", request);
        let req = request.into_inner();

        {
            let mut sessions = self.sessions_service.lock().await;
            sessions.delete_session(&req.session_token);
        }

        let reply = SignOutResponse {
            status_code: StatusCode::Success as i32,
        };

        Ok(Response::new(reply))
    }

}