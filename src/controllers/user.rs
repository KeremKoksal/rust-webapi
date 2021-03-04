use crate::models::pagination::Pagination;
use crate::models::{
  auth::Claims,
  user::{Count, Create, Delete, GetAll, GetById, PaginatedUsers, Update},
  AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

#[get("/profile/")]
pub async fn get_user_profile(claims: Claims, state: web::Data<AppState>) -> impl Responder {
  let db = state.as_ref().db.clone();

  match db
    .send(GetById {
      uid: claims.user_id,
    })
    .await
  {
    Ok(Ok(user)) => HttpResponse::Ok().json(user),
    _ => HttpResponse::InternalServerError().json("Something went wrong"),
  }
}

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
  let get_all = search.into_inner();
  let cl = get_all.clone();
  let mut _page = get_all.page.unwrap_or_default();
  if _page < 1 {
    _page = 1;
  }
  let mut _take = get_all.take.unwrap_or_default();
  if _take < 1 {
    _take = 10;
  }
  match db
    .send(Count {
      username: get_all.username,
      first_name: get_all.first_name,
      last_name: get_all.last_name,
      email: get_all.email,
      department_id: get_all.department_id,
      active: get_all.active,
      roles: get_all.roles,
    })
    .await
  {
    Ok(Ok(count)) => match db.send(cl).await {
      Ok(Ok(users)) => HttpResponse::Ok().json(PaginatedUsers {
        users: users,
        pagination: Pagination {
          page: _page,
          items_per_page: _take,
          total_count: count,
          total_pages: (count as f32 / _take as f32).ceil() as i32,
        },
      }),
      _ => HttpResponse::InternalServerError().json("Something went wrong on get all users"),
    },
    _ => HttpResponse::InternalServerError().json("Something went wrong on count"),
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
pub async fn update_user(state: web::Data<AppState>, user: web::Json<Update>) -> impl Responder {
  let db = state.as_ref().db.clone();
  let user = user.into_inner();

  match db.send(user).await {
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
