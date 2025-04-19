use crate::{db::*, error::*, middlewares::SESSION_ID_KEY, misc::extractors::ValidatedForm, templ};
use axum::response::{IntoResponse, Redirect, Response};

use super::*;
use serde::Deserialize;
use tower_sessions::Session;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginInfo {
    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 4, max = 28, message = "should be between 4-28 characters"))]
    username: String,

    #[validate(custom(function = validate_password, message = "requirements not met"))]
    password: String,
}

pub async fn login() -> ServerResult<Response> {
    templ::render(templ::Login {})
}

pub async fn login_post(
    session: Session,
    ValidatedForm(form): ValidatedForm<LoginInfo>,
) -> ServerResult<Response> {
    let Some(user) = user::fetch_one_by_username(&form.username)? else {
        return Err(ServerError::Encode("User or Password is bad"));
    };

    if !bcrypt::verify(&form.password, &user.passhash).map_err(AnyError::new)? {
        return Err(ServerError::Encode("User or Password is bad"));
    }

    session
        .insert(SESSION_ID_KEY, user.id)
        .await
        .map_err(AnyError::new)?;

    Ok(Redirect::to("/").into_response())
}
