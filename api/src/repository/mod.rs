pub mod mysql;
pub mod postgres;
pub mod user;

pub use mysql::MySQLRepositoryFactory;
pub use postgres::PostgreSQLRepositoryFactory;

use async_trait::async_trait;
use anyhow::Result;
use crate::repository::user::UserRepository;

#[async_trait]
pub trait RepositoryFactory {
    async fn create_user_repository() -> Result<Box<dyn UserRepository>>;
}
