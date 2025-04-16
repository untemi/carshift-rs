use axum::response::Redirect;
use serde::Deserialize;
use tower_sessions::Session;

pub use login::*;
pub use register::*;

mod login;
mod register;

pub async fn logout(session: Session) -> Redirect {
    session.delete().await.unwrap();
    Redirect::to("/login")
}
