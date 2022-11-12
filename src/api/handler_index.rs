use actix_web::{web, Responder};

use crate::api::interfaces::{User};

pub async fn index<T: User>(
    query: web::Path<String>,
    user: web::Data<T>,
)-> impl Responder {
    user.into_inner().do_something(&query)
}