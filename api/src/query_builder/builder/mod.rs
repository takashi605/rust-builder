pub mod mysql;
pub mod postgres;

pub trait QueryBuilder {
    fn new() -> Self;
    fn build(self) -> String;
    fn select(self, columns: Vec<&str>) -> Self;
    fn from(self, table: &str) -> Self;
}
