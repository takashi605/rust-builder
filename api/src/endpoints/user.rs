use actix_web::{get, web, HttpResponse, Responder};

use crate::repository::user::UserRepository;

#[get("/users")]
pub async fn get_users(user_repo: web::Data<Box<dyn UserRepository>>) -> impl Responder {
    let users = user_repo.find_all().await.expect("Failed to fetch users");

    HttpResponse::Ok().json(users)
}
