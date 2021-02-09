use crate::handlers::user::{Create, GetUsers,GetUser};
use crate::models::{
  user::{NewUser, SearchUser, UpdateUser},
  AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use uuid::Uuid;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/{id}")]
pub async fn get_user(state: web::Data<AppState>, web::Path(uid): web::Path<Uuid>) -> impl Responder {
  let db = state.as_ref().db.clone();
  match db.send(GetUser{uid:uid}).await{
    Ok(Ok(user)) => HttpResponse::Ok().json(user),
    _ => HttpResponse::InternalServerError().json("Something went wrong"),
  }
  
}

#[get("/")]
pub async fn get_users(
  state: web::Data<AppState>,
  search: web::Json<SearchUser>,
) -> impl Responder {
  let db = state.as_ref().db.clone();
  let search = search.into_inner();
  match db
    .send(GetUsers {
      page: search.page,
      take: search.take,
      username: search.username,
      first_name: search.first_name,
      last_name: search.last_name,
      email: search.email,
      department_id: search.department_id,
      active: search.active,
      roles: search.roles,
    })
    .await
  {
    Ok(Ok(users)) => HttpResponse::Ok().json(users),
    _ => HttpResponse::InternalServerError().json("Something went wrong"),
  }
}

#[post("/")]
pub async fn new_user(state: web::Data<AppState>, user: web::Json<NewUser>) -> impl Responder {
  let db = state.as_ref().db.clone();
  let user = user.into_inner();

  match db
    .send(Create {
      username: user.username,
      staff_title: user.staff_title,
      education_title: user.education_title,
      email: user.email,
      password: user.password,
      first_name: user.first_name,
      last_name: user.last_name,
      bio: user.bio,
      image: user.image,
      department_id: user.department_id,
      roles: user.roles,
    })
    .await
  {
    Ok(Ok(user)) => HttpResponse::Ok().json(user),
    _ => HttpResponse::InternalServerError().json("Something went wrong"),
  }
}

#[put("/")]
pub async fn update_user(pool: web::Data<DbPool>, user: web::Json<UpdateUser>) -> impl Responder {
  HttpResponse::Ok()
}

#[delete("/{id}")]
pub async fn delete_user(
  pool: web::Data<DbPool>,
  web::Path(uid): web::Path<Uuid>,
) -> impl Responder {
  HttpResponse::Ok()
}
