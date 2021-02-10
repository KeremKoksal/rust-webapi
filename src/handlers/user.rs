use super::DbActor;
use crate::diesel::prelude::*;
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use actix::{Handler, Message};
use argon2::{
  password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};
use rand_core::OsRng;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct Create {
  pub username: String,
  pub staff_title: Option<String>,
  pub education_title: Option<String>,
  pub email: String,
  pub password: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub bio: String,
  pub image: String,
  pub department_id: Option<i16>,
  pub roles: Vec<String>,
}

impl Handler<Create> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let pw: &[u8] = msg.password.as_bytes();
    let password_hash = argon2
      .hash_password_simple(pw, salt.as_ref())
      .unwrap()
      .to_string();

    let new_user = NewUser {
      username: msg.username,
      staff_title: msg.staff_title,
      education_title: msg.education_title,
      email: msg.email,
      password: password_hash,
      first_name: msg.first_name,
      last_name: msg.last_name,
      bio: msg.bio,
      image: msg.image,
      department_id: msg.department_id,
      roles: msg.roles,
    };

    diesel::insert_into(users)
      .values(new_user)
      .get_result::<User>(&conn)
  }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct GetAll {
  pub page: Option<i32>,
  pub take: Option<i32>,
  pub username: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub email: Option<String>,
  pub department_id: Option<i16>,
  pub active: Option<bool>,
  pub roles: Option<Vec<String>>,
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

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct GetById {
  pub uid: Uuid,
}

impl Handler<GetById> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, user: GetById, _: &mut Self::Context) -> Self::Result {
    let conn = self.0.get().expect("Unable to get a connection");
    users.filter(id.eq(user.uid)).get_result::<User>(&conn)
  }
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct Update {
  pub uid: Uuid,
  pub staff_title: Option<String>,
  pub education_title: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub bio: Option<String>,
  pub image: Option<String>,
  pub department_id: Option<i16>,
  pub roles: Option<Vec<String>>,
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

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct Delete {
  pub uid: Uuid,
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
