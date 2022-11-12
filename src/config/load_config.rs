use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::{env};

#[derive(Debug, Serialize,Clone,Deserialize)]
pub struct Config {
    pub environment: String,
    pub port: String,
    pub yml_cfg: ConfigYml,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ConfigYml {
    pub dynamodb: DynamoDBConfig,
}


#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DynamoDBConfig {
    pub table_name: String,
    pub primary_key: String,
    pub endpoint: String,
    pub region_name: String,
}

// load_from_file -> read both from yaml files and env files
// Load into Config object
// Initialised on boot
pub fn load_from_file() -> Config {
    let base_path = "src/config/environment/";
    
    // Read from env file
    let environment = env::var("ENV").expect("missing environment variable ENV");
    let port = env::var("PORT").expect("missing environment variable PORT");

    // Read from yml file
    let path = format!("{}{}.yml", base_path, environment);

    println!("PATH {}", path);

    let f = std::fs::File::open(path).expect("Could not open file.");
    let yml_cfg: ConfigYml = serde_yaml::from_reader(f).expect("Could not read values.");
    println!("{:?}", yml_cfg);

    let cfg = Config {
        environment,
        port,
        yml_cfg
    };

    cfg
    
}