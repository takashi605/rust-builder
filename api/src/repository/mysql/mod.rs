use crate::repository::{user::UserRepository, RepositoryFactory};
use anyhow::Result;
use async_trait::async_trait;

mod user;

pub struct MySQLRecordFactory {}

#[async_trait]
impl RepositoryFactory for MySQLRecordFactory {
    async fn create_user_repository() -> Result<Box<dyn UserRepository>> {
        user::mysql_user_repository_factory()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create MySQL user repository: {}", e))
    }
}
