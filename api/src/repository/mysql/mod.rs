use crate::repository::{user::UserRepository, RepositoryFactory};
use anyhow::Result;
use async_trait::async_trait;

mod user;

pub struct MySQLRepositoryFactory {}

#[async_trait]
impl RepositoryFactory for MySQLRepositoryFactory {
    async fn create_user_repository() -> Result<Box<dyn UserRepository>> {
        user::create_mysql_user_repository()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create MySQL user repository: {}", e))
    }
}
