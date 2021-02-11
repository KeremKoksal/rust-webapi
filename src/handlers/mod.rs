use actix::{Actor, SyncContext};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub mod user;
pub mod auth;

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);
impl Actor for DbActor {
  type Context = SyncContext<Self>;
}
