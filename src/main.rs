#[macro_use]
extern crate diesel;
use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};

mod handlers;
mod models;
mod routes;
mod controllers;
mod schema;

use handlers::DbActor;
use models::AppState;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use actix::SyncArbiter;

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("Status: up ")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let bd_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let db_manager = ConnectionManager::<PgConnection>::new(bd_url);

    let db_pool = Pool::builder()
        .build(db_manager)
        .expect("Failed to create pool");

    let db_addr = SyncArbiter::start(4, move || DbActor(db_pool.clone()));

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                db: db_addr.clone()
            })
            .service(web::scope("/api/v1").configure(routes::config))
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .service(status)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
