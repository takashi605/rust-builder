use crate::repository::user::{UserRecord, UserRepository};
use anyhow::Result;
use async_trait::async_trait;

pub struct MySQLUserRepository {
    pool: sqlx::Pool<sqlx::MySql>,
}

impl MySQLUserRepository {
    pub fn new(pool: sqlx::Pool<sqlx::MySql>) -> Self {
        MySQLUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for MySQLUserRepository {
    async fn find_all(&self) -> Result<Vec<UserRecord>> {
        let users = sqlx::query_as::<_, UserRecord>("SELECT id, name, email FROM users")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch users: {}", e))?;
        Ok(users)
    }

    async fn create_or_update(&self, user: UserRecord) -> Result<()> {
        sqlx::query("INSERT INTO users (name, email) VALUES (?, ?) ON DUPLICATE KEY UPDATE name = VALUES(name)")
            .bind(&user.name)
            .bind(&user.email)
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create or update user: {}", e))?;
        Ok(())
    }
}

pub async fn create_mysql_user_repository() -> Result<Box<dyn UserRepository>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment variables");

    let pool = sqlx::Pool::<sqlx::MySql>::connect(&database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to MySQL: {}", e))?;

    Ok(Box::new(MySQLUserRepository::new(pool)))
}
