use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub age: String,
    //pub created_at: DateTime<Utc>,
}


// User Create request for POST -> Create User
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserCreateRequest {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: String,
}