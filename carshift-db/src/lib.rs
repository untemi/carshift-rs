use anyhow::{anyhow, Context, Result};
use chrono::NaiveDate;
use db_ref::{DbRef, FillDbRef};
use sqlx::{migrate::MigrateDatabase, prelude::FromRow, sqlite::SqlitePoolOptions, SqlitePool};
use tokio::sync::OnceCell;
use tower_sessions_sled_store::SledStore;

pub mod car;
pub mod db_ref;
pub mod user;

static APPDB_PATH: &str = "sqlite://app.db";

#[derive(Default, Clone, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub passhash: String,
    pub firstname: String,
    pub lastname: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub pfp_file: Option<String>,
}

#[derive(FromRow)]
pub struct Car {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub owner: DbRef<User>,
    pub district: i64,
    pub pic_file: String,
}

#[derive(Debug, Default, FromRow)]
pub struct District {
    pub id: u8,
    pub name: String,
}

impl FillDbRef<i64> for DbRef<User> {
    async fn fill(&mut self) -> anyhow::Result<()> {
        if let Self::Ref(id) = self {
            let user = user::fetch_one_by_id(*id).await?.ok_or(anyhow!("none"))?;
            *self = Self::Some(user);
        }

        Ok(())
    }
}

static POOL: OnceCell<SqlitePool> = OnceCell::const_new();
static DISTRICTS: OnceCell<Box<[District]>> = OnceCell::const_new();

pub fn pool() -> Result<&'static SqlitePool> {
    POOL.get().context("DB not initialized")
}

pub fn districts() -> Result<&'static [District]> {
    let districts = DISTRICTS.get().context("DB not initialized")?;
    Ok(districts)
}

pub async fn init() -> Result<()> {
    if !sqlx::Sqlite::database_exists(APPDB_PATH).await? {
        sqlx::Sqlite::create_database(APPDB_PATH).await?;
    }

    let pool = SqlitePoolOptions::new()
        .test_before_acquire(false)
        .connect(APPDB_PATH)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    DISTRICTS.set(
        sqlx::query_as("SELECT * FROM districts")
            .fetch_all(&pool)
            .await?
            .into_boxed_slice(),
    )?;

    POOL.set(pool)?;
    Ok(())
}

pub async fn build_session_store() -> anyhow::Result<SledStore> {
    let sled = sled::open("session.db").unwrap();
    let store = SledStore::new(sled.open_tree("session").unwrap());
    Ok(store)
}
