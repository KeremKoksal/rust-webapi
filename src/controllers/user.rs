use crate::models::{
  user::{Create, GetAll,Delete, GetById, Update },
  AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

#[get("/{id}")]
pub async fn get_user(
  state: web::Data<AppState>,
  web::Path(uid): web::Path<Uuid>,
) -> impl Responder {
  let db = state.as_ref().db.clone();
  match db.send(GetById { uid: uid }).await {
    Ok(Ok(user)) => HttpResponse::Ok().json(user),
    _ => HttpResponse::InternalServerError().json("Something went wrong"),
  }
}

#[get("/")]
pub async fn get_users(state: web::Data<AppState>, search: web::Json<GetAll>) -> impl Responder {
  let db = state.as_ref().db.clone();
  let search = search.into_inner();
  match db.send(search).await {
    Ok(Ok(users)) => HttpResponse::Ok().json(users),
    _ => HttpResponse::InternalServerError().json("Something went wrong"),
  }
}

#[post("/")]
pub async fn new_user(state: web::Data<AppState>, user: web::Json<Create>) -> impl Responder {
  let db = state.as_ref().db.clone();
  let user = user.into_inner();

  match db.send(user).await {
    Ok(Ok(user)) => HttpResponse::Created().json(user),
    _ => HttpResponse::InternalServerError().json("Something went wrong"),
  }
}

#[put("/")]
pub async fn update_user(
  state: web::Data<AppState>,
  user: web::Json<Update>,
) -> impl Responder {
  let db = state.as_ref().db.clone();
  let user = user.into_inner();

  match db
    .send(user)
    .await
  {
    Ok(Ok(user)) => HttpResponse::Ok().json(user),
    _ => HttpResponse::InternalServerError().json("Something went wrong"),
  }
}

#[delete("/{id}")]
pub async fn delete_user(
  state: web::Data<AppState>,
  web::Path(uid): web::Path<Uuid>,
) -> impl Responder {
  let db = state.as_ref().db.clone();
  match db.send(Delete { uid: uid }).await {
    Ok(Ok(user)) => HttpResponse::Ok().json(user),
    _ => HttpResponse::InternalServerError().json("Something went wrong"),
  }
}
