use chrono::NaiveDate;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Serialize;
use std::sync::LazyLock;

pub mod car;
pub mod user;

#[derive(Default, Clone, PartialEq, Eq)]
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

#[derive(Default, Serialize)]
pub struct Car {
    pub id: u64,
    pub name: String,
    pub price: f64,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub owner: u64,
    pub district: u64,
}

#[derive(Debug, Default)]
pub struct District {
    pub id: u8,
    pub name: String,
}

pub static POOL: LazyLock<Pool<SqliteConnectionManager>> = LazyLock::new(|| {
    let manager = SqliteConnectionManager::file("app.db");
    r2d2::Pool::new(manager).unwrap()
});

pub static DISTRICTS: LazyLock<Box<[District]>> = LazyLock::new(|| {
    let conn = POOL.get().unwrap();
    let query = r#"SELECT * FROM districts"#;

    let mut stmt = conn.prepare(query).unwrap();
    let districts = stmt
        .query_map([], |r| {
            Ok(District {
                id: r.get(0)?,
                name: r.get(1)?,
            })
        })
        .unwrap()
        .filter_map(anyhow::Result::ok)
        .collect();

    districts
});
