use actix_web::{web, Responder};

use crate::api::interfaces::{UserSvc};

pub async fn index<T: UserSvc>(
    query: web::Path<String>,
    user: web::Data<T>,
)-> impl Responder {
    user.into_inner().do_something(&query)
}