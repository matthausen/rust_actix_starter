pub trait UserSvc { // API interface
    fn do_something(&self, query: &str) -> String;
}