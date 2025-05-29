use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use sqlx::prelude::FromRow;

#[async_trait]
pub trait RepositoryFactory {
    async fn user_repo_factory() -> Result<Box<dyn UserRepository>>;
}

#[derive(Serialize, FromRow, Debug)]
pub struct UserRecord {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<UserRecord>>;
}
