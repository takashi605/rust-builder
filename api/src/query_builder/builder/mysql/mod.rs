struct MysqlQueryBuilder {
    query: String,
}

impl MysqlQueryBuilder {
    pub fn new() -> Self {
        MysqlQueryBuilder {
            query: String::new(),
        }
    }

    pub fn build(self) -> String {
        self.query
    }

    pub fn select(mut self, columns: Vec<&str>) -> Self {
        let columns_str = columns.join(", ");
        self.query = format!("SELECT {}", columns_str);
        self
    }

    pub fn from(mut self, table: &str) -> Self {
        self.query = format!("{} FROM {}", self.query, table);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_build_select_users_query() {
        let query = MysqlQueryBuilder::new()
            .select(vec!["id", "name", "email"])
            .from("users")
            .build();
        assert_eq!(query, "SELECT id, name, email FROM users");
    }
}