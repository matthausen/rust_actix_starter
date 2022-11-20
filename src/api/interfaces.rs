use crate::users::storage::model::{User};

pub trait UserSvc { // API interface
    fn do_something(&self, query: &str) -> String;
    fn create(&self, to_create: User) -> User; // return pointer to model or error
}