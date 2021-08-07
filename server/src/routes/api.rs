use actix_web::{get, post, web, HttpResponse, Responder};
use crate::{models};

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg
        .service(hello)
        .service(echo)
        .service(web::resource("/api/add_user").route(web::post().to(models::user::User::add_user)))
        .route("/hey", web::get().to(manual_hello))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::ServiceUnavailable().body("Request error")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
