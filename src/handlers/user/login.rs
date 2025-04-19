use crate::{db::*, middlewares::SESSION_ID_KEY, misc::extractors::ValidatedForm, templ};
use axum::response::{Html, IntoResponse, Redirect, Response};

use super::*;
use askama::Template;
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

pub async fn login() -> Html<String> {
    Html(templ::Login {}.render().unwrap())
}

pub async fn login_post(
    session: Session,
    ValidatedForm(form): ValidatedForm<LoginInfo>,
) -> Response {
    let Some(user) = user::fetch_one_by_username(&form.username).unwrap() else {
        return "User or Password is bad".into_response();
    };

    if !bcrypt::verify(&form.password, &user.passhash).unwrap() {
        return "User or Password is bad".into_response();
    }

    session.insert(SESSION_ID_KEY, user.id).await.unwrap();
    Redirect::to("/").into_response()
}
