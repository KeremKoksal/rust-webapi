mod users;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(users::user_routes());
}
