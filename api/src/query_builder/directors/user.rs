use crate::query_builder::builder::mysql::MysqlQueryBuilder;

pub struct SelectUsersDirector {
    builder: MysqlQueryBuilder,
}
impl SelectUsersDirector {
    pub fn new(builder: MysqlQueryBuilder) -> Self {
        SelectUsersDirector {
            builder,
        }
    }

    pub fn build_query(self) -> String {
        self.builder
            .select(vec!["id", "name", "email"])
            .from("users")
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_users_director() {
        let builder = MysqlQueryBuilder::new();
        let director = SelectUsersDirector::new(builder);
        let query = director.build_query();
        assert_eq!(query, "SELECT id, name, email FROM users");
    }
}
