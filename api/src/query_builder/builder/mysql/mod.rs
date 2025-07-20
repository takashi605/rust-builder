// ステートフルな構造体のため Clone を実装
// Mutex+Arc 等を使用しても良いが、そこまで大きなデータは扱わないのでシンプルに Clone で対応
#[derive(Clone)]
pub struct MysqlQueryBuilder {
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

    #[test]
    fn test_clone_query_builder() {
        let builder = MysqlQueryBuilder::new()
            .select(vec!["id", "name", "email"])
            .from("users");
        let cloned_builder = builder.clone();
        assert_eq!(builder.build(), cloned_builder.build());
    }
}