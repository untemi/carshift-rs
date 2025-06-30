use crate::hx_redirect;
use cs_middlewares::SESSION_ID_KEY;
use cs_misc::extractors::ValidatedForm;
use db::*;

use super::*;
use axum::response::Response;
use serde::Deserialize;
use tower_sessions::Session;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterInfo {
    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 3, max = 15, message = "should be between 3-15 characters"))]
    username: String,

    #[validate(custom(function = validate_password, message = "requirements not met"))]
    password: String,

    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 2, max = 30, message = "should be between 2-30 characters"))]
    firstname: String,

    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 2, max = 30, message = "should be between 2-30 characters"))]
    lastname: Option<String>,
}

pub async fn register() -> ServerResult<Response> {
    templ::render(templ::Register {})
}

pub async fn register_post(
    session: Session,
    ValidatedForm(form): ValidatedForm<RegisterInfo>,
) -> ServerResult<Response> {
    // structuring new user from form
    let user = User {
        username: form.username,
        firstname: form.firstname,
        lastname: form.lastname,
        passhash: bcrypt::hash(form.password, 4).map_err(AnyError::new)?,
        ..Default::default()
    };

    // is username used
    if user::is_username_used(&user.username)? {
        return Err(ServerError::Encode("Username already used"));
    }

    // registering user and grabbing their id
    let id = user::register(user)?;

    // to the session boy
    session
        .insert(SESSION_ID_KEY, id)
        .await
        .map_err(AnyError::new)?;

    Ok(hx_redirect("/"))
}
