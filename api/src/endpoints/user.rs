use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
    // モックデータの作成
    let users = vec![
        User {
            id: 1,
            name: "山田 太郎".to_string(),
            email: "taro.yamada@example.com".to_string(),
        },
        User {
            id: 2,
            name: "佐藤 花子".to_string(),
            email: "hanako.sato@example.com".to_string(),
        },
    ];

    HttpResponse::Ok().json(users)
}