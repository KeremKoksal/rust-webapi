use crate::controllers::user::{delete_user, get_user, get_users, new_user, update_user,get_user_profile};

pub fn user_routes() -> actix_web::Scope {
  actix_web::web::scope("/users")
    .service(get_user)
    .service(get_users)
    .service(new_user)
    .service(update_user)
    .service(delete_user)
    .service(get_user_profile)
}
