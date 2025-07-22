use crate::{query_builder::{builder::{mysql::MysqlQueryBuilder, QueryBuilder}, directors::{select_user::SelectUsersDirector, upsert_user::UpsertUserDirector}}, repository::user::{UserRecord, UserRepository}};
use anyhow::Result;
use async_trait::async_trait;

pub struct MySQLUserRepository {
    pool: sqlx::Pool<sqlx::MySql>,
    builder: MysqlQueryBuilder,
}

impl MySQLUserRepository {
    pub fn new(pool: sqlx::Pool<sqlx::MySql>, builder: MysqlQueryBuilder) -> Self {
        MySQLUserRepository { pool, builder }
    }
}

#[async_trait]
impl UserRepository for MySQLUserRepository {
    async fn find_all(&self) -> Result<Vec<UserRecord>> {
        let director = SelectUsersDirector::new(self.builder.clone());
        let query = director.build_query();

        let users = sqlx::query_as::<_, UserRecord>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch users: {}", e))?;
        Ok(users)
    }

    async fn create_or_update(&self, user: UserRecord) -> Result<()> {
        let upsert_query_director = UpsertUserDirector::new(self.builder.clone());
        let upsert_query = upsert_query_director.build_query(&user.name, &user.email);

        sqlx::query(&upsert_query)
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
    let query_builder = MysqlQueryBuilder::new();

    Ok(Box::new(MySQLUserRepository::new(pool, query_builder)))
}
