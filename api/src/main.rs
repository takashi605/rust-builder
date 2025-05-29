mod endpoints;
mod repository;

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use std::sync::Arc;

use crate::repository::mysql::user::mysql_user_repository_factory;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server at http://0.0.0.0:8000");

    // リポジトリを作成
    let user_repository = mysql_user_repository_factory()
        .await
        .expect("Failed to create user repository");

    // リポジトリをArcでラップして共有できるようにする
    let user_repository = Arc::new(user_repository);

    HttpServer::new(move || {
        let user_repository = Arc::clone(&user_repository);

        App::new()
             // 各ハンドラ関数で共有可能なデータとしてリポジトリを追加
            .app_data(web::Data::new(user_repository))
            .service(endpoints::root_scope())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}
