use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::Endpoint;

mod config;
mod db;
mod api;
mod users;

use crate::config::load_config::load_from_file;
use crate::config::load_config::Config;

use crate::db::dynamodb::create_table_if_not_exists;

use crate::api::handler_index::index;
use crate::api::handler_create_user::{create_user};
use crate::api::interfaces::UserSvc;

use crate::users::storage::service::{Storage};
use crate::users::service::{UserService};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg: Config = load_from_file();

    // init DynamoDB client
    let aws_config = aws_config::load_from_env().await;
    let dynamodb_local_aws_config = aws_sdk_dynamodb::config::Builder::from(&aws_config)
        .endpoint_resolver(
            Endpoint::immutable(actix_web::http::Uri::from_static("http://localhost:8000")),
        )
        .build();
    let client = Client::from_conf(dynamodb_local_aws_config);
    
    // Create the table if it doesn't exist -> TODO -> use result
    create_table_if_not_exists(&cfg, &client).await;

    // Services init
    let user_storage: Storage = Storage::new(&cfg.yml_cfg.dynamodb.table_name);
    let user_service = UserService::new();
    create_server(user_service).unwrap().await
}


pub fn create_server<T: UserSvc + Send + Sync + 'static + Clone>(
    user_svc: T,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .data(user_svc.clone())
            .route("/{query}", web::get().to(index::<T>))
            .route("/user", web::post().to(create_user::<T>))
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}