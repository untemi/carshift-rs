use super::Car;
use crate::pool;
use chrono::NaiveDate;
use sqlx::QueryBuilder;

pub async fn fetch_one(id: i64) -> anyhow::Result<Option<Car>> {
    let query = "SELECT * FROM cars WHERE id=?1 LIMIT 1";
    let car: Option<Car> = sqlx::query_as(query).bind(id).fetch_optional(pool()?).await?;
    Ok(car)
}

pub async fn find_many(
    input: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    district: Option<i64>,
    offset: i64,
    limit: u8,
) -> anyhow::Result<Box<[Car]>> {
    let mut builder = QueryBuilder::new("SELECT * FROM cars");

    // Add WHERE clause if needed
    if input.is_some() || (start_date.is_some() && end_date.is_some()) || district.is_some() {
        builder.push(" WHERE ");
    }

    let mut first_condition = true;

    // Handle input search
    if let Some(input) = input {
        let words: Vec<&str> = input.split_whitespace().collect();
        for word in words {
            if !first_condition {
                builder.push(" AND ");
            }
            builder.push("name LIKE ").push_bind(format!("%{word}%"));
            first_condition = false;
        }
    }

    // Handle date range
    if let (Some(start_date), Some(end_date)) = (start_date, end_date) {
        if !first_condition {
            builder.push(" AND ");
        }
        builder.push("start_date <= ").push_bind(start_date);
        builder.push(" AND end_date >= ").push_bind(end_date);
        first_condition = false;
    }

    // Handle district filter
    if let Some(district) = district {
        if !first_condition {
            builder.push(" AND ");
        }
        builder.push("district = ").push_bind(district);
    }

    // Add LIMIT and OFFSET
    builder.push(" LIMIT ").push_bind(limit as i64);
    builder.push(" OFFSET ").push_bind(offset);

    let cars = builder.build_query_as().fetch_all(pool()?).await?;
    Ok(cars.into_boxed_slice())
}
