use super::POOL;
use super::User;
use r2d2_sqlite::rusqlite::{OptionalExtension, params};

impl User {
    pub fn new() -> Self {
        Self {
            id: 0,
            username: String::new(),
            passhash: String::new(),
            firstname: String::new(),
            lastname: None,
            email: None,
            phone: None,
            pfp_file: None,
        }
    }
}

pub fn register(user: User) -> anyhow::Result<u64> {
    let conn = POOL.get()?;
    let query = r#"
        INSERT INTO users (username,passhash,firstname,lastname)
            VALUES (?1,?2,?3,?4)
            RETURNING id
    "#;

    let id: u64 = conn.query_row(
        query,
        params![user.username, user.passhash, user.firstname, user.lastname],
        |r| r.get(0),
    )?;
    Ok(id)
}

pub fn fetch_one_by_username(username: &String) -> anyhow::Result<Option<User>> {
    let conn = POOL.get()?;
    let query = "SELECT * FROM users WHERE username=?1 LIMIT 1";

    let user: Option<User> = conn
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
    let conn = POOL.get()?;
    let query = "SELECT * FROM users WHERE id=?1 LIMIT 1";

    let user: Option<User> = conn
        .query_row(query, [id], |r| {
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

pub fn is_username_used(username: &String) -> anyhow::Result<bool> {
    let conn = POOL.get()?;
    let query = r#"
        SELECT COUNT(id) FROM users
            WHERE username = ?1
            LIMIT 1;
    "#;

    let count: u64 = conn.query_row(query, [username], |r| r.get(0))?;
    Ok(count == 1)
}
