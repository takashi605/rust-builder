use crate::query_builder::builder::mysql::MysqlQueryBuilder;

struct SelectUsersDirector {
    query: String,
}
impl SelectUsersDirector {
    pub fn new() -> Self {
        SelectUsersDirector {
            query: String::new(),
        }
    }

    pub fn build_query(&self) -> String {
        MysqlQueryBuilder::new()
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
        let director = SelectUsersDirector::new();
        let build_query = director.build_query();
        assert_eq!(build_query, "SELECT id, name, email FROM users");
    }
}
