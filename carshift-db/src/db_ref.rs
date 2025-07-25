use anyhow::Result;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{Sqlite, SqliteValueRef};
use sqlx::{decode::Decode, Type};

pub enum DbRef<B> {
    Ref(i64),
    Some(B),
}

pub trait FillDbRef<A> {
    fn fill(&mut self) -> impl Future<Output = anyhow::Result<()>>;
}

impl<T> Type<Sqlite> for DbRef<T> {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
}

impl<'r, T> sqlx::Decode<'r, Sqlite> for DbRef<T> {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let id = <i64 as Decode<Sqlite>>::decode(value)?;
        Ok(DbRef::Ref(id))
    }
}
