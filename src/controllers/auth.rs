use crate::models::auth::Claims;
use crate::models::{
  auth::{Auth, Login},
  AppState,
};
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{
  Argon2, {PasswordHash, PasswordVerifier},
};
use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header,Algorithm};

#[post("/login")]
pub async fn login(state: web::Data<AppState>, data: web::Json<Login>) -> impl Responder {
  let db = state.as_ref().db.clone();
  let jwt = state.as_ref().jwt.clone();
  let search = data.into_inner();
  let login_attempt = search.clone();
  match db.send(search).await {
    Ok(Ok(user)) => {
      let found_user = user.clone();
      let found_password = user.password.unwrap();
      let argon2 = Argon2::default();

      let parsed_hash = PasswordHash::new(&found_password).unwrap();

      if argon2
        .verify_password(login_attempt.password.unwrap().as_bytes(), &parsed_hash)
        .is_ok()
      {
        let mut headers = Header::default();
        headers.alg = Algorithm::HS512;
        let encoding_key = EncodingKey::from_secret(jwt.as_bytes());
        let now = Utc::now() + Duration::days(1); // Expires in 1 day
        let claims = Claims {
          user_id: user.id,
          exp: now.timestamp(),
          roles: user.roles,
        };

        let response = Auth {
          token: encode(&headers, &claims, &encoding_key).unwrap(),
          user: found_user
        };
        HttpResponse::Ok().json(response)
      } else {
        HttpResponse::Unauthorized().json("Invalid username or password")
      }
    }
    _ => HttpResponse::Unauthorized().json("Invalid username or password"),
  }
}
