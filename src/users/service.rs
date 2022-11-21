use async_trait::async_trait;
use aws_sdk_dynamodb::model::{AttributeValue};
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::error::{PutItemError};

use aws_sdk_dynamodb::{Client};

use crate::api::interfaces::{UserSvc};
use crate::users::model::{User};
use chrono::{Utc};


#[async_trait]
pub trait UserStorage {
    //async fn add_item(&self, user: User) -> User;
}


#[derive(Clone)]
pub struct UserService {
    table_name: String,
    client: Client
}

impl UserService {
    pub fn new(client: Client, table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            client
        }
    }
}

#[async_trait]
impl UserSvc for UserService {
    fn do_something(&self, query: &str) -> String {
        format!("Doing something in the UserService: {}", query)
    }

    async fn add_item(&self, item: User) -> Result<(), SdkError<PutItemError>> { // return pointer to user or error
        
        let user_av = AttributeValue::S(item.first_name);
        let type_av = AttributeValue::S(item.last_name);
        let age_av = AttributeValue::S(item.age);
        let first_av = AttributeValue::S(item.email);
        let last_av = AttributeValue::S(item.password);

        let request = self.client
            .put_item()
            .table_name(self.table_name.to_string())
            .item("id", user_av)
            .item("email", type_av)
            .item("age", age_av)
            .item("first_name", first_av)
            .item("last_name", last_av);

        println!("Executing request [{request:?}] to add item...");

        let resp = request.send().await?;
        println!("We do get here {:?}", resp);

        //let attributes = resp.attributes().unwrap();

        // println!(
        //     "Added user {:?}, {:?} {:?}, age {:?} as {:?} user",
        //     attributes.get("username"),
        //     attributes.get("first_name"),
        //     attributes.get("last_name"),
        //     attributes.get("age"),
        //     attributes.get("p_type")
        // );

        Ok(())
        
    }
}

/* 

return User { 
            id: String::from("123"), 
            first_name: String::from("123"), 
            last_name: String::from("123"), 
            email: String::from("123"), 
            password: String::from("123"), 
            age: String::from("123") 
        }
*/