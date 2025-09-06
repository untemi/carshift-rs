use super::Car;
use crate::{pool, Sortings};
use chrono::NaiveDate;
use sqlx::QueryBuilder;

pub async fn add(
    name: String,
    price: f64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    user_id: i64,
    district: i64,
    description: Option<String>,
    pic_file: String,
) -> anyhow::Result<i64> {
    let query = r#"
        INSERT INTO cars (name,price,start_date,end_date,owner,district,description,pic_file)
            VALUES (?,?,?,?,?,?,?,?)
            RETURNING id
    "#;

    let (id,): (i64,) = sqlx::query_as(query)
        .bind(name)
        .bind(price)
        .bind(start_date)
        .bind(end_date)
        .bind(user_id)
        .bind(district)
        .bind(description)
        .bind(pic_file)
        .fetch_one(pool()?)
        .await?;

    Ok(id)
}

pub async fn fetch_one(id: i64) -> anyhow::Result<Option<Car>> {
    let query = "SELECT * FROM cars WHERE id=? LIMIT 1";
    let car: Option<Car> = sqlx::query_as(query).bind(id).fetch_optional(pool()?).await?;
    Ok(car)
}

pub async fn count_from_user(id: i64) -> anyhow::Result<i64> {
    let query = "SELECT COUNT(*) FROM cars WHERE owner = ?";
    let (count,) = sqlx::query_as(query).bind(id).fetch_one(pool()?).await?;
    Ok(count)
}

pub async fn fetch_from_user(
    id: i64,
    sort: Sortings,
    offset: i64,
    limit: u8,
) -> anyhow::Result<Box<[Car]>> {
    let query = format!(
        "SELECT * FROM cars WHERE owner=? ORDER BY {} LIMIT ? OFFSET ?",
        sort.into_clause()
    );

    let cars = sqlx::query_as(&query)
        .bind(id)
        .bind(limit)
        .bind(offset * limit as i64)
        .fetch_all(pool()?)
        .await?;

    Ok(cars.into_boxed_slice())
}

pub async fn find_many(
    input: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    district: Option<i64>,
    sort: Option<Sortings>,
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

    if let Some(sort) = sort {
        builder.push(format!(" ORDER BY {} ", sort.into_clause()));
    }

    // Add LIMIT and OFFSET
    builder.push(" LIMIT ").push_bind(limit as i64);
    builder.push(" OFFSET ").push_bind(offset * limit as i64);

    let cars = builder.build_query_as().fetch_all(pool()?).await?;
    Ok(cars.into_boxed_slice())
}
