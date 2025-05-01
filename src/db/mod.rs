use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Serialize;
use std::sync::LazyLock;

pub mod user;

#[derive(Default, Serialize, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub passhash: String,
    pub firstname: String,
    pub lastname: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub pfp_file: Option<String>,
}

pub static POOL: LazyLock<Pool<SqliteConnectionManager>> = LazyLock::new(|| {
    let manager = SqliteConnectionManager::file("app.db");
    r2d2::Pool::new(manager).unwrap()
});
