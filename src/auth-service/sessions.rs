use uuid::Uuid;
use std::collections::HashMap;

pub trait Sessions {
    fn create_session(&mut self, user_uuid: &str) -> String;
    fn delete_session(&mut self, user_uuid: &str);
}

#[derive(Debug, Clone)]
pub struct SessionsInstance {
    pub uuid_to_session: HashMap<String, String>, // Maps user_uuid to session_uuid
}

impl SessionsInstance {
    pub fn new() -> Self {
        Self {
            uuid_to_session: HashMap::new(),
        }
    }
}

impl Sessions for SessionsInstance {

    fn create_session(&mut self, user_uuid: &str) -> String {
        let session = Uuid::new_v4().to_string();
        self.uuid_to_session
            .insert(user_uuid.to_owned(), session.clone());
        session
    }

    fn delete_session(&mut self, user_uuid: &str) {
        if self.uuid_to_session.contains_key(user_uuid) {
            self.uuid_to_session.remove(user_uuid);
        }
    }

}