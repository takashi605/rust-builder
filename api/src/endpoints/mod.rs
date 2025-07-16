mod user;

use actix_web::{web, Scope};

pub fn root_scope() -> Scope {
    web::scope("")
        .service(user::get_users)
        .service(user::create_user)
}
