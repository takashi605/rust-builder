use crate::repository::user::{UserRepository, UserRecord};

pub struct MySQLUserRepository {}

impl UserRepository for MySQLUserRepository {
    fn find_all(&self) -> Vec<UserRecord> {
        vec![
            UserRecord {
                id: 1,
                name: "山田 太郎".to_string(),
                email: "taro.yamada@example.com".to_string(),
            },
            UserRecord {
                id: 2,
                name: "佐藤 花子".to_string(),
                email: "hanako.sato@example.com".to_string(),
            },
        ]
    }
}

pub fn mysql_user_repository_factory() -> Box<dyn UserRepository> {
    Box::new(MySQLUserRepository {})
}
