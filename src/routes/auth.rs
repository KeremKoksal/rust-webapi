use crate::controllers::auth::login;

pub fn auth_routes() -> actix_web::Scope {
  actix_web::web::scope("/auth").service(login)
}
