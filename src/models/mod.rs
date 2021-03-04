pub mod auth;
pub mod user;
pub mod pagination;
use crate::handlers::DbActor;
use actix::Addr;

pub struct AppState {
  pub db: Addr<DbActor>,
  pub jwt: String
}
