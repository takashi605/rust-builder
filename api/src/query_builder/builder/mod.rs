pub mod mysql;
pub mod postgres;

pub trait QueryBuilder {
    fn new() -> Self;
    fn build(self) -> String;

    // Select 用のメソッド
    fn select(self, columns: Vec<&str>) -> Self;
    fn from(self, table: &str) -> Self;

    // Insert 用のメソッド
    fn insert(self, table: &str) -> Self;
    fn columns(self, columns: Vec<&str>) -> Self;
    fn values(self, values: Vec<&str>) -> Self;

    // upsert 用のメソッド
    fn on_conflict(self, conflict_column: &str) -> Self;
    fn do_update(self, update_columns: &str) -> Self;
}
