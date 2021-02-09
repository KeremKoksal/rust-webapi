use diesel::prelude::*;

use crate::models::user::User;
use crate::schema::users::dsl::*;
use diesel::PgConnection;
use uuid::Uuid;

pub fn find_user_by_uuid(
  uid: Uuid,
  conn: &PgConnection,
) -> Result<Option<User>, diesel::result::Error> {
  let user = users.filter(id.eq(uid)).first::<User>(conn).optional()?;
  Ok(user)
}

pub fn get_all_user(conn: &PgConnection) -> Result<Option<Vec<User>>, diesel::result::Error> {
  let user = users.load::<User>(conn).optional()?;
  Ok(user)
}
