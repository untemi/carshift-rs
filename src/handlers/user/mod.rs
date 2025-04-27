use crate::{error::*, fancy_validator};
use axum::response::Redirect;
use lazy_static::lazy_static;
use regex::Regex;
use tower_sessions::Session;

pub use login::*;
pub use picture::*;
pub use register::*;
pub use settings::*;

mod login;
mod picture;
mod register;
mod settings;

fancy_validator!(password, r"^(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}");

lazy_static! {
    static ref REGEX_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    static ref REGEX_NAME: Regex = Regex::new(r"^[\p{L}\p{Zs}'-]+$").unwrap();
    static ref REGEX_PHONE: Regex =
        Regex::new(r"^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$").unwrap();
}

pub async fn logout(session: Session) -> ServerResult<Redirect> {
    session.delete().await.map_err(AnyError::new)?;
    Ok(Redirect::to("/login"))
}
