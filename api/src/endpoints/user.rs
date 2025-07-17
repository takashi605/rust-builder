use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::repository::user::{UserRecord, UserRepository};

#[get("/users")]
pub async fn get_users(user_repo: web::Data<Box<dyn UserRepository>>) -> impl Responder {
    let users = user_repo.find_all().await.expect("Failed to fetch users");

    HttpResponse::Ok().json(users)
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[post("/users")]
pub async fn create_user(
    user_repo: web::Data<Box<dyn UserRepository>>,
    req: web::Json<CreateUserRequest>,
) -> impl Responder {
    let user = UserRecord {
        id: 0,
        name: req.name.clone(),
        email: req.email.clone(),
    };

    match user_repo.create_or_update(user).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"message": "User created or updated successfully"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("Failed to create or update user: {}", e)})),
    }
}
