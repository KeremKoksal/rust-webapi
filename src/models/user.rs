use crate::models::pagination::Pagination;
use crate::schema::users;
use actix::Message;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;
use diesel::{prelude::*, Insertable, Queryable};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub staff_title: Option<String>,
  pub education_title: Option<String>,
  pub email: String,
  #[serde(skip_serializing)]
  pub password: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub bio: Option<String>,
  pub image: Option<String>,
  pub department_id: Option<i16>,
  #[serde(skip_serializing)]
  pub email_verified: bool,
  #[serde(skip_serializing)]
  pub active: bool,
  pub roles: Vec<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl Responder for User {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();

    ready(Ok(
      HttpResponse::Ok()
        .content_type("application/json")
        .body(body),
    ))
  }
}

#[derive(Serialize)]
pub struct PaginatedUsers {
  pub users: Vec<User>,
  pub pagination: Pagination
}


#[derive(Message, Deserialize, Clone)]
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
#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<i64>")]
pub struct Count {
  pub username: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub email: Option<String>,
  pub department_id: Option<i16>,
  pub active: Option<bool>,
  pub roles: Option<Vec<String>>,
}


#[derive(Message, Deserialize, Debug, Insertable)]
#[rtype(result = "QueryResult<User>")]
#[table_name = "users"]
pub struct Create {
  pub username: String,
  pub staff_title: Option<String>,
  pub education_title: Option<String>,
  pub email: String,
  pub password: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub bio: String,
  pub image: String,
  pub department_id: Option<i16>,
  pub roles: Vec<String>,
}
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct GetById {
  pub uid: Uuid,
}

#[derive(Message, Deserialize, Debug)]
#[rtype(result = "QueryResult<User>")]
pub struct Update {
  pub id: Uuid,
  pub staff_title: Option<String>,
  pub education_title: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub bio: Option<String>,
  pub image: Option<String>,
  pub department_id: Option<i16>,
  pub roles: Option<Vec<String>>,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct Delete {
  pub uid: Uuid,
}
