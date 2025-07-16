use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct UserRecord {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<UserRecord>>;
    async fn create(&self, user: UserRecord) -> Result<()>;
}
