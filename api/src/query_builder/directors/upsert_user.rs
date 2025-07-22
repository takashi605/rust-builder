use crate::query_builder::builder::QueryBuilder;

pub struct UpsertUserDirector<B: QueryBuilder> {
    builder: B,
}

impl<B: QueryBuilder> UpsertUserDirector<B> {
    pub fn new(builder: B) -> Self {
        UpsertUserDirector { builder }
    }

    pub fn build_query(self, name: &str, email: &str) -> String {
        self.builder
            .insert("users")
            .columns(vec!["name", "email"])
            .values(vec![name, email])
            .on_conflict("email")
            .do_update(vec!["name"])
            .build()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_upsert_user_director_with_mysql() {
        use crate::query_builder::builder::mysql::MysqlQueryBuilder;

        let builder = MysqlQueryBuilder::new();
        let director = UpsertUserDirector::new(builder);
        let query = director.build_query("takashi", "takashi@example.com");

        assert_eq!(query, "INSERT INTO users (name, email) VALUES ('takashi', 'takashi@example.com') ON DUPLICATE KEY UPDATE name = VALUES(name)");
    }

    #[test]
    fn test_upsert_user_director_with_postgres() {
        use crate::query_builder::builder::{postgres::PostgresQueryBuilder, QueryBuilder};

        let builder = PostgresQueryBuilder::new();
        let director = UpsertUserDirector::new(builder);
        let query = director.build_query("takashi", "takashi@example.com");

        assert_eq!(query, "INSERT INTO users (name, email) VALUES ('takashi', 'takashi@example.com') ON CONFLICT (email) DO UPDATE SET name = EXCLUDED.name");
    }

}