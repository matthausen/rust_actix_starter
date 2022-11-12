use actix_web::{web, HttpResponse, Responder, error, Error};
use futures::StreamExt;

use crate::api::interfaces::{UserSvc};
use crate::users::storage::model::{UserCreateRequest, User};

const MAX_SIZE: usize = 262_144; // max payload size is 256k


pub async fn create_user<T: UserSvc>(
    mut query: web::Payload,
    user: web::Data<T>,
)-> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = query.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let server_req = serde_json::from_slice::<UserCreateRequest>(&body)?;

    let user = user.into_inner().create(&server_req);
    Ok(HttpResponse::Ok().json(user))
}