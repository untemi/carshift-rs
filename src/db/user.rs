use super::{POOL, User};
use r2d2_sqlite::rusqlite::{OptionalExtension, params};

pub fn update(user: User) -> anyhow::Result<()> {
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

    POOL.get()?.execute(
        query,
        params![
            user.username,
            user.firstname,
            user.lastname,
            user.pfp_file,
            user.passhash,
            user.phone,
            user.email,
            user.id
        ],
    )?;

    Ok(())
}

pub fn register(user: User) -> anyhow::Result<u64> {
    let query = r#"
        INSERT INTO users (username,passhash,firstname,lastname)
            VALUES (?1,?2,?3,?4)
            RETURNING id
    "#;

    let id: u64 = POOL.get()?.query_row(
        query,
        params![user.username, user.passhash, user.firstname, user.lastname],
        |r| r.get(0),
    )?;
    Ok(id)
}

pub fn fetch_one_by_username(username: &str) -> anyhow::Result<Option<User>> {
    let query = "SELECT * FROM users WHERE username=?1 LIMIT 1";

    let user: Option<User> = POOL
        .get()?
        .query_row(query, [username], |r| {
            Ok(User {
                id: r.get(0)?,
                username: r.get(1)?,
                passhash: r.get(2)?,
                firstname: r.get(3)?,
                lastname: r.get(4)?,
                email: r.get(5)?,
                phone: r.get(6)?,
                pfp_file: r.get(7)?,
            })
        })
        .optional()?;

    Ok(user)
}

pub fn fetch_one_by_id(id: u64) -> anyhow::Result<Option<User>> {
    let query = "SELECT * FROM users WHERE id=?1 LIMIT 1";

    let user: Option<User> = POOL
        .get()?
        .query_row(query, [id], |r| {
            Ok(User {
                id: r.get(0)?,
                username: r.get(1)?,
                passhash: r.get(2)?,
                firstname: r.get(3)?,
                lastname: r.get(4)?,
                phone: r.get(5)?,
                email: r.get(6)?,
                pfp_file: r.get(7)?,
            })
        })
        .optional()?;

    Ok(user)
}

pub fn is_username_used(username: &str) -> anyhow::Result<bool> {
    let query = "SELECT COUNT(id) FROM users WHERE username = ?1 LIMIT 1";

    let count: u64 = POOL.get()?.query_row(query, [username], |r| r.get(0))?;
    Ok(count == 1)
}

pub fn update_picture(id: u64, path: &str) -> anyhow::Result<()> {
    let conn = POOL.get()?;
    let query = r#"UPDATE users SET pfp_file = ?1 WHERE id = ?2"#;

    conn.execute(query, params![path, id])?;
    Ok(())
}

pub fn find_many(input: &str, offset: u64, limit: u8) -> anyhow::Result<Box<[User]>> {
    let query = r#" SELECT * FROM users WHERE username LIKE ?1 LIMIT ?3 OFFSET ?2"#;

    let users = POOL
        .get()?
        .prepare(query)?
        .query_map(params![format!("%{input}%"), offset, limit], |r| {
            Ok(User {
                id: r.get(0)?,
                username: r.get(1)?,
                passhash: r.get(2)?,
                firstname: r.get(3)?,
                lastname: r.get(4)?,
                email: r.get(5)?,
                phone: r.get(6)?,
                pfp_file: r.get(7)?,
            })
        })?
        .filter_map(anyhow::Result::ok)
        .collect();

    Ok(users)
}
