use crate::query_builder::builder::QueryBuilder;

pub struct SelectUsersDirector<B: QueryBuilder> {
    builder: B,
}
impl<B: QueryBuilder> SelectUsersDirector<B> {
    pub fn new(builder: B) -> Self {
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
    use crate::query_builder::builder::mysql::MysqlQueryBuilder;

    #[test]
    fn test_select_users_director_with_mysql() {
        let builder = MysqlQueryBuilder::new();
        let director = SelectUsersDirector::new(builder);
        let query = director.build_query();
        assert_eq!(query, "SELECT id, name, email FROM users");
    }
}
