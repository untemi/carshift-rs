use crate::hx_redirect;
use csutils::extractors::ValidatedForm;
use db::*;
use mw::SESSION_ID_KEY;

use super::*;
use axum::response::Response;
use serde::Deserialize;
use tower_sessions::Session;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginInfo {
    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 3, max = 15, message = "should be between 3-15 characters"))]
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
    // verifying if user exist
    let Some(user) = user::fetch_one_by_username(&form.username)? else {
        return Err(ServerError::Encode("User or Password is bad"));
    };

    // if user exists verify password by bcrypt compare
    if !bcrypt::verify(&form.password, &user.passhash).map_err(AnyError::new)? {
        return Err(ServerError::Encode("User or Password is bad"));
    }

    // to the session boy
    session
        .insert(SESSION_ID_KEY, user.id)
        .await
        .map_err(AnyError::new)?;

    Ok(hx_redirect("/"))
}
