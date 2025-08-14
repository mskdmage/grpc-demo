use std::collections::HashMap;

use pbkdf2::{
    Pbkdf2,
    password_hash::{SaltString, PasswordHasher, PasswordHash, PasswordVerifier},
};

use rand_core::OsRng;

use uuid::Uuid;

pub trait Users {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String>;
    fn get_user_uuid(&self, username: String, password: String) -> Option<String>;
    fn delete_user(&mut self, user_uuid: String);
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_uuid: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UsersInstance {
    pub uuid_to_user: HashMap<String, User>,
    pub username_to_user: HashMap<String, User>,
}

impl UsersInstance {
    pub fn new() -> Self {
        Self {
            uuid_to_user: HashMap::new(),
            username_to_user: HashMap::new(),
        }
    }
}

impl Users for UsersInstance {

    fn create_user(&mut self, username: String, password: String) -> Result<(), String> {
        
        if self.username_to_user.contains_key(&username) {
            return Err(format!("User {} already exists", username));
        }

        let salt = SaltString::generate(OsRng);

        let hashed_password = Pbkdf2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();

        let user = User {
            user_uuid: Uuid::new_v4().to_string(),
            username,
            password: hashed_password,
        };

        self.uuid_to_user.insert(user.user_uuid.clone(), user.clone());
        self.username_to_user.insert(user.username.clone(), user.clone());

        Ok(())
    }

    fn get_user_uuid(&self, username: String, password: String) -> Option<String> {
        
        let user = self.username_to_user.get(&username)?;

        let hashed_password = user.password.clone();

        let parsed_hash = PasswordHash::new(&hashed_password).ok()?;

        let result = Pbkdf2.verify_password(password.as_bytes(), &parsed_hash);

        match result {
            Ok(_) => Some(user.user_uuid.clone()),
            Err(_) => None,
        }
    }

    fn delete_user(&mut self, user_uuid: String) {
        if let Some(user) = self.uuid_to_user.remove(&user_uuid) {
            self.username_to_user.remove(&user.username);
        }
    }
}