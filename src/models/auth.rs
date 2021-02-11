use actix::Message;
use diesel::prelude::*;
use serde::{Serialize,Deserialize};
use crate::models::user::User;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Auth {
  pub token: String,
  pub user: User
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub roles: Vec<String>
}


#[derive(Message, Deserialize,Queryable,Serialize,Clone)]
#[rtype(result = "QueryResult<User>")]
pub struct Login {
  pub username: String,
  pub password: Option<String>,
}
