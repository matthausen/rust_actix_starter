use aws_sdk_dynamodb::client::fluent_builders::{PutItem};
use aws_sdk_dynamodb::model::{AttributeValue};
use std::io::Error;
use async_trait::async_trait;

use crate::users::service::UserStorage;
use crate::users::storage::model::{User};

pub trait Dynamo {
    fn put_item(&self) -> PutItem;
}


#[derive(Clone)]
pub struct Storage {
    app_version: String,
    table_name: String,
}

impl Storage {
    pub fn new(table_name: &str) -> Self {
        Self {
            app_version: String::from("1337"),
            table_name: table_name.to_string(),
        }
    }
}

#[async_trait]
impl UserStorage for Storage {
    async fn add_item(&self, item: User) -> Result<(), Error> {
        let user_av = AttributeValue::S(item.first_name);
        let type_av = AttributeValue::S(item.last_name);
        let age_av = AttributeValue::S(item.age);
        let first_av = AttributeValue::S(item.email);
        let last_av = AttributeValue::S(item.password);

        let request = client
            .put_item()
            .table_name(self.table_name)
            .item("username", user_av)
            .item("account_type", type_av)
            .item("age", age_av)
            .item("first_name", first_av)
            .item("last_name", last_av);

        println!("Executing request [{request:?}] to add item...");

        let resp = request.send().await?;

        let attributes = resp.attributes().unwrap();

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