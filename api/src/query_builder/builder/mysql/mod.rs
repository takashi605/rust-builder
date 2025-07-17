struct MysqlQueryBuilder {
    query: String,
}

impl MysqlQueryBuilder {
    pub fn new() -> Self {
        MysqlQueryBuilder {
            query: String::new(),
        }
    }

    pub fn build(mut self) -> String {
        self.query = String::from("SELECT id, name, email FROM users");
        self.query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_select_users_query() {
        let query = MysqlQueryBuilder::new()
            .build();
        assert_eq!(query, "SELECT id, name, email FROM users");
    }
}