use crate::api::interfaces::{UserSvc};
use crate::users::storage::model::{User, UserCreateRequest};
use chrono::{Utc};


#[derive(Clone)]
pub struct UserService {
    app_version: String
}

impl UserService {
    pub fn new() -> Self {
        Self {
            app_version: String::from("1337"),
        }
    }
}

impl UserSvc for UserService {
    fn do_something(&self, query: &str) -> String {
        format!("Doing something in the UserService: {}", query)
    }

    fn create(&self, to_create: &UserCreateRequest) -> User {
        // do something
        User { 
            id: String::from("123"), 
            first_name: String::from("John"), 
            last_name: String::from("Wick"), 
            email: String::from("john.wick@example.com"), 
            password: String::from("****hidden****"), 
            age: String::from("32"), 
        }
    }
}

