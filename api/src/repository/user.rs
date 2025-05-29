use serde::Serialize;

#[derive(Serialize)]
pub struct UserRecord {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub trait UserRepository {
    fn find_all(&self) -> Vec<UserRecord>;
}