mod endpoints;
mod repository;

use actix_web::{web, App, HttpServer};
use anyhow::Result;

use crate::repository::mysql::MySQLRecordFactory;
use crate::repository::user::RepositoryFactory;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server at http://0.0.0.0:8000");

    // リポジトリを作成
    let user_repo = MySQLRecordFactory::user_repo_factory()
        .await
        .expect("Failed to create user repository");

    // リポジトリをアプリケーションのデータとして共有するためにラップ
    // web::Data は Arc を内包しているため、スレッドセーフで共有可能
    let user_repo = web::Data::new(user_repo);

    HttpServer::new(move || {
        App::new()
            // 各ハンドラ関数で共有可能なデータとしてリポジトリを追加
            .app_data(user_repo.clone())
            .service(endpoints::root_scope())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}
