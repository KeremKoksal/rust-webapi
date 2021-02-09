pub mod user;
use crate::handlers::DbActor;
use actix::Addr;

pub struct AppState {
  pub db: Addr<DbActor>
}