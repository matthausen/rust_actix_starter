use crate::users::model::{User};
use async_trait::async_trait;
use aws_sdk_dynamodb::error::{PutItemError};
use aws_sdk_dynamodb::types::SdkError;


#[async_trait]
pub trait UserSvc { // API interface
    fn do_something(&self, query: &str) -> String;
    async fn add_item(&self, to_create: User) -> Result<(), SdkError<PutItemError>>; // return pointer to model or error
}