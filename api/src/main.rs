mod endpoints;
mod repository;

use actix_web::{web, App, HttpServer};
use anyhow::Result;

use crate::repository::mysql::MySQLRecordFactory;
use crate::repository::postgres::PostgreSQLRecordFactory;
use crate::repository::user::RepositoryFactory;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server at http://0.0.0.0:8000");

    // 環境変数から使用するDBを取得
    let usage_db = std::env::var("USAGE_DB").unwrap_or_else(|_| "mysql".to_string());
    println!("Using database: {}", usage_db);

    // DBタイプに応じたリポジトリを作成
    let user_repo = match usage_db.as_str() {
        "postgres" => PostgreSQLRecordFactory::user_repo_factory().await?,
        _ => MySQLRecordFactory::user_repo_factory().await?, // デフォルトはMySQL
    };

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
