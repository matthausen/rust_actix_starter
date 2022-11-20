use async_trait::async_trait;


use crate::api::interfaces::{UserSvc};
use crate::users::storage::model::{User};
use crate::users::storage::service::{Storage};
use chrono::{Utc};


#[async_trait]
pub trait UserStorage {
    async fn add_item(&self, user: User) -> User;
}


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

    fn create(&self, to_create: User) -> User { // return pointer to user or error
        // do something
        let result = self::Storage::add_item(&self::Storage, to_create);

        return result
    }
}

