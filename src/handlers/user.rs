use super::DbActor;
use crate::models::user::{Count, Create, Delete, GetAll, GetById, Update, User};
use crate::schema::users::dsl::*;
use actix::Handler;
use argon2::{
  password_hash::{PasswordHasher, SaltString},
  Argon2,
};
use diesel::dsl::count;
use diesel::prelude::*;
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
    if let Some(_limit) = search_filters.take {
      limit = _limit;
    }

    if let Some(page) = search_filters.page {
      if page > 0 {
        offset = (page - 1) * limit;
      }
    };

    let mut data = users.offset(offset.into()).limit(limit.into()).into_boxed();

    if let Some(_username) = search_filters.username {
      data = data.filter(username.ilike(format!("%{}%", _username)));
    }
    if let Some(_first_name) = search_filters.first_name {
      data = data.filter(first_name.ilike(format!("%{}%", _first_name)));
    }
    if let Some(_last_name) = search_filters.last_name {
      data = data.filter(last_name.ilike(format!("%{}%", _last_name)));
    }
    if let Some(_email) = search_filters.email {
      data = data.filter(email.ilike(format!("%{}%", _email)));
    }
    if let Some(_department_id) = search_filters.department_id {
      data = data.filter(department_id.eq(_department_id));
    }
    if let Some(_active) = search_filters.active {
      data = data.filter(active.eq(_active));
    }
    if let Some(_roles) = search_filters.roles {
      data = data.filter(roles.contains(_roles));
    }

    data.get_results::<User>(&conn)
  }
}
impl Handler<Count> for DbActor {
  type Result = QueryResult<i64>;

  fn handle(&mut self, search_filters: Count, _: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");

    let mut data = users.into_boxed();

    if let Some(_username) = search_filters.username {
      data = data.filter(username.ilike(format!("%{}%", _username)));
    }
    if let Some(_first_name) = search_filters.first_name {
      data = data.filter(first_name.ilike(format!("%{}%", _first_name)));
    }
    if let Some(_last_name) = search_filters.last_name {
      data = data.filter(last_name.ilike(format!("%{}%", _last_name)));
    }
    if let Some(_email) = search_filters.email {
      data = data.filter(email.ilike(format!("%{}%", _email)));
    }
    if let Some(_department_id) = search_filters.department_id {
      data = data.filter(department_id.eq(_department_id));
    }
    if let Some(_active) = search_filters.active {
      data = data.filter(active.eq(_active));
    }
    if let Some(_roles) = search_filters.roles {
      data = data.filter(roles.contains(_roles));
    }

    data.select(count(id)).first(&conn)
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
    diesel::update(users.filter(id.eq(user.id)))
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
