mod user;

use actix_web::{web, Scope};

pub fn root_scope() -> Scope {
    web::scope("")
        .service(hello)
        .service(user::get_users)
}

use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello, world!!")
}
