use aws_sdk_dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use std::process;
use aws_sdk_dynamodb::{Client, Error};

use crate::config::load_config::Config;

// Create table with AWS SDK for DynamoDB if it doesn't exists already
pub async fn create_table_if_not_exists(
    cfg: &Config,
    client: &Client, 
    ) -> Result<(), Error> {
    let a_name: String = String::from(&cfg.yml_cfg.dynamodb.primary_key);
    let table_name: String = String::from(&cfg.yml_cfg.dynamodb.table_name);

    let ad = AttributeDefinition::builder()
        .attribute_name(&a_name)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(&a_name)
        .key_type(KeyType::Hash)
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    match client
        .create_table()
        .table_name(table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await
    {
        Ok(_) => println!("Added table {} with key {}", &cfg.yml_cfg.dynamodb.table_name, &cfg.yml_cfg.dynamodb.primary_key),
        Err(e) => {
            println!("Could not create table or table already exists: {}", e);
        }
    };

    Ok(())
}