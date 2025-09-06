use super::User;
use crate::pool;

pub async fn update(user: User) -> anyhow::Result<()> {
    let query = r#"
        UPDATE users 
            SET username = ?,
            firstname = ?,
            lastname = ?,
            pfp_file = ?,
            passhash = ?,
            phone = ?,
            email = ?
            WHERE id = ?;
        "#;

    sqlx::query(query)
        .bind(user.username)
        .bind(user.firstname)
        .bind(user.lastname)
        .bind(user.pfp_file)
        .bind(user.passhash)
        .bind(user.phone)
        .bind(user.email)
        // where
        .bind(user.id)
        .execute(pool()?)
        .await?;

    Ok(())
}

pub async fn register(user: User) -> anyhow::Result<u64> {
    let query = r#"
        INSERT INTO users (username,firstname,lastname,passhash)
            VALUES (?,?,?,?)
            RETURNING id
    "#;

    let (id,) = sqlx::query_as(query)
        .bind(&user.username)
        .bind(&user.firstname)
        .bind(&user.lastname)
        .bind(&user.passhash)
        .fetch_one(pool()?)
        .await?;

    Ok(id)
}

pub async fn fetch_one_by_username(username: &str) -> anyhow::Result<Option<User>> {
    let query = "SELECT * FROM users WHERE username=? LIMIT 1";
    let user: Option<User> = sqlx::query_as(query)
        .bind(username)
        .fetch_optional(pool()?)
        .await?;

    Ok(user)
}

pub async fn fetch_one_by_id(id: i64) -> anyhow::Result<Option<User>> {
    let query = "SELECT * FROM users WHERE id=? LIMIT 1";
    let user: Option<User> = sqlx::query_as(query).bind(id).fetch_optional(pool()?).await?;
    Ok(user)
}

pub async fn is_username_used(username: &str) -> anyhow::Result<bool> {
    let query = "SELECT COUNT(id) FROM users WHERE username = ? LIMIT 1";
    let (count,): (u64,) = sqlx::query_as(query).bind(username).fetch_one(pool()?).await?;
    Ok(count > 0)
}

pub async fn update_picture(id: i64, path: &str) -> anyhow::Result<()> {
    let query = r#"UPDATE users SET pfp_file = ? WHERE id = ?"#;
    sqlx::query(query).bind(path).bind(id).execute(pool()?).await?;
    Ok(())
}

pub async fn find_many(input: &str, offset: i64, limit: u8) -> anyhow::Result<Box<[User]>> {
    let query = r#" SELECT * FROM users WHERE username LIKE ? LIMIT ? OFFSET ?"#;
    let users: Vec<User> = sqlx::query_as(query)
        .bind(format!("%{input}%"))
        .bind(limit)
        .bind(offset * limit as i64)
        .fetch_all(pool()?)
        .await?;
    Ok(users.into_boxed_slice())
}
