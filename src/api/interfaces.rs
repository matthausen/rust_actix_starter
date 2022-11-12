use crate::users::storage::model::{User, UserCreateRequest};



pub trait UserSvc { // API interface
    fn do_something(&self, query: &str) -> String;
    fn create(&self, to_create: &UserCreateRequest) -> User;
}