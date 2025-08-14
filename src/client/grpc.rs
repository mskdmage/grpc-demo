mod authentication {
    tonic::include_proto!("authentication");
}

pub mod auth {
    
    pub use super::authentication::{
        SignInRequest,
        SignInResponse,
        SignOutRequest,
        SignOutResponse,
        SignUpRequest,
        SignUpResponse,
    };

    pub use super::authentication::auth_client::AuthClient;

}