use crate::middlewares::SESSION_ID_KEY;
use axum::response::Redirect;
use serde::Deserialize;
use tower_sessions::Session;

pub use login::*;
pub use register::*;
mod login;
mod register;

#[derive(Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

pub async fn logout(session: Session) -> Redirect {
    session.delete().await.unwrap();
    Redirect::to("/login")
}
