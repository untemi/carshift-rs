use crate::{db::*, error::*, middlewares::SESSION_ID_KEY, misc::extractors::ValidatedForm, templ};

use super::*;
use axum::response::{IntoResponse, Redirect, Response};
use serde::Deserialize;
use tower_sessions::Session;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterInfo {
    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 4, max = 28, message = "should be between 4-28 characters"))]
    username: String,

    #[validate(custom(function = validate_password, message = "requirements not met"))]
    password: String,

    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 2, max = 30, message = "should be between 4-28 characters"))]
    firstname: String,

    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 2, max = 30, message = "should be between 4-28 characters"))]
    lastname: Option<String>,
}

pub async fn register() -> ServerResult<Response> {
    templ::render(templ::Register {})
}

pub async fn register_post(
    session: Session,
    ValidatedForm(form): ValidatedForm<RegisterInfo>,
) -> ServerResult<Response> {
    let mut user = User::new();
    user.username = form.username;
    user.firstname = form.firstname;
    user.lastname = form.lastname;
    user.passhash = bcrypt::hash(form.password, 4).map_err(AnyError::new)?;

    if user::is_username_used(&user.username)? {
        return Err(ServerError::Encode("Username already used"));
    }

    let id = user::register(user)?;

    session
        .insert(SESSION_ID_KEY, id)
        .await
        .map_err(AnyError::new)?;

    Ok(Redirect::to("/").into_response())
}
