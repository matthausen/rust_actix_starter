use actix_web::{web, HttpResponse, error, Error};
use futures::StreamExt;

use crate::api::interfaces::{UserSvc};
use crate::users::model::{UserCreateRequest, User};

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

    let to_create = parse_user_req_to_use_model(server_req);

    let user = user.into_inner().add_item(to_create).await;
    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::BadRequest().json("error"))
    }
    
}


fn parse_user_req_to_use_model(serv_req: UserCreateRequest) -> User {
    User {
        id: serv_req.id,
        first_name: serv_req.first_name,
        last_name: serv_req.last_name,
        email: serv_req.email,
        password: String::from("MUST_REPLACE"),
        age: serv_req.age,
    }
}