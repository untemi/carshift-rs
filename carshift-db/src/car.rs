use super::{Car, DbRef, POOL};
use chrono::NaiveDate;
use r2d2_sqlite::rusqlite::{OptionalExtension, ToSql, params_from_iter};

pub fn fetch_one(id: u64) -> anyhow::Result<Option<Car>> {
    let query = "SELECT * FROM cars WHERE id=?1 LIMIT 1";

    let user: Option<Car> = POOL
        .get()?
        .query_row(query, [id], |r| {
            Ok(Car {
                id: r.get(0)?,
                name: r.get(1)?,
                price: r.get(2)?,
                start_date: r.get(3)?,
                end_date: r.get(4)?,
                owner: DbRef::Ref(r.get(5)?),
                district: r.get(6)?,
            })
        })
        .optional()?;

    Ok(user)
}

pub fn find_many(
    input: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    district: Option<u64>,
    offset: u64,
    limit: u8,
) -> anyhow::Result<Box<[Car]>> {
    let mut query = String::from("SELECT * FROM cars");
    let mut where_clauses = Vec::new();
    let mut params_vec: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(input) = input {
        for word in input.split_whitespace() {
            where_clauses.push("name LIKE ?");
            params_vec.push(Box::new(format!("%{word}%")));
        }
    }

    if let (Some(start_date), Some(end_date)) = (start_date, end_date) {
        where_clauses.push("start_date <= ?");
        where_clauses.push("end_date >= ?");

        params_vec.push(Box::new(start_date));
        params_vec.push(Box::new(end_date));
    }

    if let Some(district) = district {
        where_clauses.push("district = ?");
        params_vec.push(Box::new(district));
    }

    if !where_clauses.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&where_clauses.join(" AND "));
    }

    query.push_str(" LIMIT ? OFFSET ?");
    params_vec.push(Box::new(limit));
    params_vec.push(Box::new(offset));

    let users = POOL
        .get()?
        .prepare(&query)?
        .query_map(params_from_iter(params_vec.into_iter()), |r| {
            Ok(Car {
                id: r.get(0)?,
                name: r.get(1)?,
                price: r.get(2)?,
                start_date: r.get(3)?,
                end_date: r.get(4)?,
                owner: DbRef::Ref(r.get(5)?),
                district: r.get(6)?,
            })
        })?
        .filter_map(anyhow::Result::ok)
        .collect();

    Ok(users)
}
