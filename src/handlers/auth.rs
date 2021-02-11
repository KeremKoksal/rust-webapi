use super::DbActor;
use crate::models::{auth::Login, user::User};
use crate::schema::users::dsl::*;

use actix::Handler;
use diesel::prelude::*;

impl Handler<Login> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, login: Login, _: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    users
      .filter(username.eq(login.username))
      .get_result::<User>(&conn)
  }
}
