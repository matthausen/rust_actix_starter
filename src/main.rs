use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

mod api;
mod users;

use crate::api::handler_index::index;
use crate::api::interfaces::User;
use crate::users::service::{UserService};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_service = UserService::new();
    create_server(user_service).unwrap().await
}


pub fn create_server<T: User + Send + Sync + 'static + Clone>(
    user_svc: T,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .data(user_svc.clone())
            .route("/{query}", web::get().to(index::<T>))
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}