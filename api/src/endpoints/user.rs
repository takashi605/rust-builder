use actix_web::{get, HttpResponse, Responder};

use crate::repository::mysql::user::mysql_user_repository_factory;

#[get("/users")]
pub async fn get_users() -> impl Responder {
    let repository = mysql_user_repository_factory();
    let users = repository
        .find_all();

    HttpResponse::Ok().json(users)
}
