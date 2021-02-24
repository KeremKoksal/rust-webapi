mod auth;
mod users;

use crate::middleware::auth::bearer_validator;
use actix_cors::Cors;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(
      users::user_routes()
        .wrap(HttpAuthentication::bearer(bearer_validator))
        .wrap(Cors::permissive()),
    )
    .service(auth::auth_routes());
}
