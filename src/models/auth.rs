use crate::models::user::User;
use actix::Message;
use actix_web::{
  dev::Payload,
  error::{ErrorInternalServerError, ErrorUnauthorized},
  Error, FromRequest, HttpRequest,
};

use actix_web_httpauth::extractors::bearer::BearerAuth;
use diesel::prelude::*;
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{
  errors::ErrorKind,
  {decode, Algorithm, DecodingKey, Validation},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Auth {
  pub token: String,
  pub user: User,
}

#[derive(Message, Deserialize, Queryable, Serialize, Clone)]
#[rtype(result = "QueryResult<User>")]
pub struct Login {
  pub username: String,
  pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
  pub user_id: Uuid,
  pub exp: i64,
  pub roles: Vec<String>,
}

impl FromRequest for Claims {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;
  type Config = ();
  fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
    let bearer_result = BearerAuth::from_request(req, payload).into_inner();
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET");

    match bearer_result {
      Ok(bearer) => {
        let token_data = match decode::<Claims>(
          &bearer.token(),
          &DecodingKey::from_secret(jwt_secret.as_bytes()),
          &Validation::new(Algorithm::HS512),
        ) {
          Ok(c) => c,
          Err(e) => match *e.kind() {
            ErrorKind::InvalidToken => return err(ErrorUnauthorized("Invalid Token")),
            ErrorKind::ExpiredSignature => {
              return err(ErrorUnauthorized(format!("Token Expired: {}", e)))
            }
            _ => {
              return err(ErrorInternalServerError(format!(
                "Internal Server Error: {}",
                e
              )))
            }
          },
        };
        ok(token_data.claims)
      }
      _ => err(ErrorUnauthorized("Unauthorized Access")),
    }
  }
}