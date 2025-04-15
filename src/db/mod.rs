use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::LazyLock;

pub mod user;

pub struct User {
    pub id: u64,
    pub username: String,
    pub passhash: String,
}

pub static POOL: LazyLock<Pool<SqliteConnectionManager>> = LazyLock::new(|| {
    let manager = SqliteConnectionManager::file("app.db");
    r2d2::Pool::new(manager).unwrap()
});
