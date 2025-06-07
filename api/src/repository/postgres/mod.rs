use crate::repository::{user::UserRepository, RepositoryFactory};
use anyhow::Result;
use async_trait::async_trait;

mod user;

pub struct PostgreSQLRepositoryFactory {}

#[async_trait]
impl RepositoryFactory for PostgreSQLRepositoryFactory {
    async fn create_user_repository() -> Result<Box<dyn UserRepository>> {
        user::create_postgres_user_repository()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create PostgreSQL user repository: {}", e))
    }
}
