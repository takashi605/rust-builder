mod endpoints;
mod repository;

use actix_web::{App, HttpServer};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server at http://0.0.0.0:8000");

    HttpServer::new(|| App::new().service(endpoints::root_scope()))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await?;

    Ok(())
}
