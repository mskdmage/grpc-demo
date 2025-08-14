use tokio::sync::Mutex;

use crate::auth::AuthService;
use crate::{users::{Users, UsersInstance}, sessions::{Sessions, SessionsInstance}};
use crate::grpc::auth::{Auth, SignInRequest, SignUpRequest, SignOutRequest, StatusCode};

#[test]
fn should_create_user() {
    let mut user_service = UsersInstance::new();
    user_service
        .create_user("username".to_owned(), "password".to_owned())
        .expect("should create user");

    assert_eq!(user_service.uuid_to_user.len(), 1);
    assert_eq!(user_service.username_to_user.len(), 1);
}

#[test]
fn should_fail_creating_user_with_existing_username() {
    let mut user_service = UsersInstance::new();
    user_service
        .create_user("username".to_owned(), "password".to_owned())
        .expect("should create user");

    let result = user_service.create_user("username".to_owned(), "password".to_owned());

    assert!(result.is_err());
}

#[test]
fn should_retrieve_user_uuid() {
    let mut user_service = UsersInstance::new();
    user_service
        .create_user("username".to_owned(), "password".to_owned())
        .expect("should create user");

    assert!(user_service
        .get_user_uuid("username".to_owned(), "password".to_owned())
        .is_some());
}

#[test]
fn should_fail_to_retrieve_user_uuid_with_incorrect_password() {
    let mut user_service = UsersInstance::new();
    user_service
        .create_user("username".to_owned(), "password".to_owned())
        .expect("should create user");

    assert!(user_service
        .get_user_uuid("username".to_owned(), "incorrect password".to_owned())
        .is_none());
}

#[test]
fn should_delete_user() {
    let mut user_service = UsersInstance::new();
    user_service
        .create_user("username".to_owned(), "password".to_owned())
        .expect("should create user");

    let user_uuid = user_service
        .get_user_uuid("username".to_owned(), "password".to_owned())
        .unwrap();

    user_service.delete_user(user_uuid);

    assert_eq!(user_service.uuid_to_user.len(), 0);
    assert_eq!(user_service.username_to_user.len(), 0);
}

#[test]
fn should_create_session() {
    let mut session_service = SessionsInstance::new();
    assert_eq!(session_service.uuid_to_session.len(), 0);
    
    let session = session_service.create_session("123456");
    
    assert_eq!(session_service.uuid_to_session.len(), 1);
    
    // Check that the user_uuid maps to the correct session_uuid
    let mapped_session_uuid = session_service.uuid_to_session.get("123456").unwrap();
    assert_eq!(mapped_session_uuid, &session);
}

#[test]
fn should_delete_session() {
    let mut session_service = SessionsInstance::new();
    session_service.create_session("123456");
    session_service.delete_session("123456");
    assert_eq!(session_service.uuid_to_session.len(), 0);
}


#[tokio::test]
async fn sign_in_should_fail_if_user_not_found() {
    let users_service: Box<Mutex<dyn Users + Send + Sync + 'static>> = Box::new(Mutex::new(UsersInstance::new()));
    let sessions_service: Box<Mutex<dyn Sessions + Send + Sync + 'static>> = Box::new(Mutex::new(SessionsInstance::new()));

    let auth_service = AuthService::new(users_service, sessions_service);

    let request = tonic::Request::new(SignInRequest {
        username: "123456".to_owned(),
        password: "654321".to_owned(),
    });

    let result = auth_service.sign_in(request).await.unwrap().into_inner();

    assert_eq!(result.status_code, StatusCode::Failure as i32);
    assert_eq!(result.user_uuid.is_empty(), true);
    assert_eq!(result.session_token.is_empty(), true);
}

#[tokio::test]
async fn sign_in_should_fail_if_incorrect_password() {
    let mut users_service = UsersInstance::new();

    let _ = users_service.create_user("123456".to_owned(), "654321".to_owned());

    let users_service = Box::new(Mutex::new(users_service));
    let sessions_service = Box::new(Mutex::new(SessionsInstance::new()));

    let auth_service = AuthService::new(users_service, sessions_service);

    let request = tonic::Request::new(SignInRequest {
        username: "123456".to_owned(),
        password: "wrong password".to_owned(),
    });

    let result = auth_service.sign_in(request).await.unwrap().into_inner();

    assert_eq!(result.status_code, StatusCode::Failure as i32);
    assert_eq!(result.user_uuid.is_empty(), true);
    assert_eq!(result.session_token.is_empty(), true);
}

#[tokio::test]
async fn sign_in_should_succeed() {
    let mut users_service = UsersInstance::new();

    let _ = users_service.create_user("123456".to_owned(), "654321".to_owned());

    let users_service = Box::new(Mutex::new(users_service));
    let sessions_service = Box::new(Mutex::new(SessionsInstance::new()));

    let auth_service = AuthService::new(users_service, sessions_service);

    let request = tonic::Request::new(SignInRequest {
        username: "123456".to_owned(),
        password: "654321".to_owned(),
    });

    let result = auth_service.sign_in(request).await.unwrap().into_inner();

    assert_eq!(result.status_code, StatusCode::Success as i32);
    assert_eq!(result.user_uuid.is_empty(), false);
    assert_eq!(result.session_token.is_empty(), false);
}

#[tokio::test]
async fn sign_up_should_fail_if_username_exists() {
    let mut users_service = UsersInstance::new();

    let _ = users_service.create_user("123456".to_owned(), "654321".to_owned());

    let users_service = Box::new(Mutex::new(users_service));
    let sessions_service = Box::new(Mutex::new(SessionsInstance::new()));

    let auth_service = AuthService::new(users_service, sessions_service);

    let request = tonic::Request::new(SignUpRequest {
        username: "123456".to_owned(),
        password: "654321".to_owned(),
    });

    let result = auth_service.sign_up(request).await.unwrap();

    assert_eq!(result.into_inner().status_code, StatusCode::Failure as i32);
}

#[tokio::test]
async fn sign_up_should_succeed() {
    let users_service = Box::new(Mutex::new(UsersInstance::new()));
    let sessions_service = Box::new(Mutex::new(SessionsInstance::new()));

    let auth_service = AuthService::new(users_service, sessions_service);

    let request = tonic::Request::new(SignUpRequest {
        username: "123456".to_owned(),
        password: "654321".to_owned(),
    });

    let result = auth_service.sign_up(request).await.unwrap();

    assert_eq!(result.into_inner().status_code, StatusCode::Success as i32);
}

#[tokio::test]
async fn sign_out_should_succeed() {
    let users_service = Box::new(Mutex::new(UsersInstance::new()));
    let sessions_service = Box::new(Mutex::new(SessionsInstance::new()));

    // First create a session to delete
    let session_token = {
        let mut sessions = sessions_service.lock().await;
        sessions.create_session("test_user_uuid")
    };

    let auth_service = AuthService::new(users_service, sessions_service);

    let request = tonic::Request::new(SignOutRequest {
        session_token: session_token
    });

    let result = auth_service.sign_out(request).await.unwrap();

    assert_eq!(result.into_inner().status_code, StatusCode::Success as i32);
}