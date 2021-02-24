use chrono::Utc;
use actix_web::error::{ErrorUnauthorized,ErrorInternalServerError};
use actix_web::{dev::ServiceRequest,Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{errors::ErrorKind,{decode,DecodingKey,Algorithm,Validation}};
use crate::models::auth::Claims;

pub async fn bearer_validator(
  req: ServiceRequest,
  credentials: BearerAuth
)-> Result<ServiceRequest, Error>{
  let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET");
  let token_data = match decode::<Claims>(
    &credentials.token(),
    &DecodingKey::from_secret(jwt_secret.as_bytes()),
    &Validation::new(Algorithm::HS512),
) {
    Ok(c) => c,
    Err(err) => match *err.kind() {
        ErrorKind::InvalidToken => return Err(ErrorUnauthorized(format!("Invalid Token: {}", err))), 
        ErrorKind::ExpiredSignature => return Err(ErrorUnauthorized(format!("Token Expired: {}", err))),
        _ => return Err(ErrorInternalServerError(format!("Internal Server Error: {}", err))),
    },
};
  eprintln!("{:?}",token_data.claims.roles);
  eprintln!("{:?}",token_data.claims.sub);
  eprintln!("{:?}",token_data.claims.exp);
  let date=Utc::now().timestamp();
  eprintln!("{:?}",date);
  Ok(req)
}