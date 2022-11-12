pub trait User { // API interface
    fn do_something(&self, query: &str) -> String;
}