use super::DbActor;
use crate::diesel::prelude::*;
use crate::models::user::{Create, Delete, GetAll, GetById, Update, User};
use crate::schema::users::dsl::*;
use actix::Handler;
use argon2::{
  password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};
use rand_core::OsRng;

impl Handler<Create> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, mut new_user: Create, _: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");

    if let Some(pword) = new_user.password {
      let salt = SaltString::generate(&mut OsRng);
      let argon2 = Argon2::default();
      let pw: &[u8] = pword.as_bytes();
      let password_hash = argon2
        .hash_password_simple(pw, salt.as_ref())
        .unwrap()
        .to_string();
      new_user.password = Some(password_hash.to_string());
    }

    diesel::insert_into(users)
      .values(new_user)
      .get_result::<User>(&conn)
  }
}

impl Handler<GetAll> for DbActor {
  type Result = QueryResult<Vec<User>>;

  fn handle(&mut self, search_filters: GetAll, _: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    let mut offset = 0;
    let mut limit = 10;
    if search_filters.take.is_some() {
      limit = search_filters.take.unwrap();
    }

    if search_filters.page.is_some() {
      offset = search_filters.page.unwrap() * limit;
    };

    let mut data = users.offset(offset.into()).limit(limit.into()).into_boxed();

    if search_filters.username.is_some() {
      data = data.filter(username.ilike(format!("%{}%", search_filters.username.unwrap())));
    }
    if search_filters.first_name.is_some() {
      data = data.filter(first_name.ilike(format!("%{}%", search_filters.first_name.unwrap())));
    }
    if search_filters.last_name.is_some() {
      data = data.filter(last_name.ilike(format!("%{}%", search_filters.last_name.unwrap())));
    }
    if search_filters.email.is_some() {
      data = data.filter(email.ilike(format!("%{}%", search_filters.email.unwrap())));
    }
    if search_filters.department_id.is_some() {
      data = data.filter(department_id.eq(search_filters.department_id.unwrap()));
    }
    if search_filters.active.is_some() {
      data = data.filter(active.eq(search_filters.active.unwrap()));
    }
    if search_filters.roles.is_some() {
      data = data.filter(roles.contains(search_filters.roles.unwrap()));
    }

    data.get_results::<User>(&conn)
  }
}

impl Handler<GetById> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, user: GetById, _: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    users.filter(id.eq(user.uid)).get_result::<User>(&conn)
  }
}

impl Handler<Update> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, user: Update, _: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    diesel::update(users.filter(id.eq(user.uid)))
      .set((
        staff_title.eq(user.staff_title),
        education_title.eq(user.education_title),
        first_name.eq(user.first_name),
        last_name.eq(user.last_name),
        bio.eq(user.bio),
        image.eq(user.image),
        department_id.eq(user.department_id),
        roles.eq(user.roles.unwrap()),
      ))
      .get_result::<User>(&conn)
  }
}

impl Handler<Delete> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, user: Delete, _: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    diesel::delete(users)
      .filter(id.eq(user.uid))
      .get_result::<User>(&conn)
  }
}
