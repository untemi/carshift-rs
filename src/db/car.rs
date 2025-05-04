use super::{Car, POOL};
use chrono::NaiveDate;
use r2d2_sqlite::rusqlite::{ToSql, params_from_iter};

pub fn find_many(
    input: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    district: Option<u64>,
    offset: u64,
    limit: u8,
) -> anyhow::Result<Box<[Car]>> {
    let conn = POOL.get()?;
    let mut query = String::from("SELECT * FROM cars");
    let mut where_clauses = Vec::new();
    let mut params_vec: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(input) = input {
        where_clauses.push("name LIKE ?");
        params_vec.push(Box::new(format!("%{input}%")));
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

    let mut stmt = conn.prepare(&query)?;
    let users = stmt
        .query_map(params_from_iter(params_vec.into_iter()), |r| {
            Ok(Car {
                id: r.get(0)?,
                name: r.get(1)?,
                price: r.get(2)?,
                start_date: r.get(3)?,
                end_date: r.get(4)?,
                owner: r.get(5)?,
                district: r.get(6)?,
            })
        })?
        .filter_map(anyhow::Result::ok)
        .collect();

    Ok(users)
}
