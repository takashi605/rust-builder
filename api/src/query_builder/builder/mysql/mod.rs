use crate::query_builder::builder::QueryBuilder;

// ステートフルな構造体のため Clone を実装
// Mutex+Arc 等を使用しても良いが、そこまで大きなデータは扱わないのでシンプルに Clone で対応
#[derive(Clone)]
pub struct MysqlQueryBuilder {
    query: String,
}

impl QueryBuilder for MysqlQueryBuilder {
    fn new() -> Self {
        MysqlQueryBuilder {
            query: String::new(),
        }
    }

    fn build(self) -> String {
        self.query
    }

    fn select(mut self, columns: Vec<&str>) -> Self {
        let columns_str = columns.join(", ");
        self.query = format!("SELECT {}", columns_str);
        self
    }

    fn from(mut self, table: &str) -> Self {
        self.query = format!("{} FROM {}", self.query, table);
        self
    }

    // Insert 用のメソッド

    fn insert(mut self, table: &str) -> Self {
        self.query = format!("INSERT INTO {}", table);
        self
    }

    fn columns(mut self, columns: Vec<&str>) -> Self {
        let columns_str = columns.join(", ");
        self.query = format!("{} ({})", self.query, columns_str);
        self
    }

    fn values(mut self, values: Vec<&str>) -> Self {
        let values_str = values.join(", ");
        self.query = format!("{} VALUES ({})", self.query, values_str);
        self
    }

    fn on_conflict(mut self, _conflict_column: &str) -> Self {
        self.query = format!("{} ON DUPLICATE KEY UPDATE", self.query);
        self
    }

    fn do_update(mut self, update_columns: Vec<&str>) -> Self {
        let update_str = update_columns.join(", ");
        self.query = format!("{} {} = VALUES({})", self.query, update_str, update_str);
        self
    }
}

#[cfg(test)]
mod tests {

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
    fn test_build_upsert_user_query() {
        let query = MysqlQueryBuilder::new()
            .insert("users")
            .columns(vec!["name", "email"])
            .values(vec!["takashi", "takashi@example.com"])
            .on_conflict("email")
            .do_update(vec!["name"])
            .build();

        assert_eq!(query, "INSERT INTO users (name, email) VALUES (takashi, takashi@example.com) ON DUPLICATE KEY UPDATE name = VALUES(name)");
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