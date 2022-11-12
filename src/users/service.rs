use crate::api::interfaces::{UserSvc};

#[derive(Clone)]
pub struct UserService {
    app_version: String
}

impl UserSvc for UserService {
    fn do_something(&self, query: &str) -> String {
        format!("Doing something in the UserService: {}", query)
    }
}

impl UserService { // create service
    pub fn new() -> Self {
        Self {
            app_version: String::from("1337"),
        }
    }
}