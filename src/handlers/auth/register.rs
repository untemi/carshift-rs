use crate::{
    db::*, extractors::ValidatedForm, fancy_validator, middlewares::SESSION_ID_KEY, templ,
};

use super::*;
use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use lazy_static::lazy_static;
use regex::Regex;
use tower_sessions::Session;
use validator::Validate;

fancy_validator!(password, r"^(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}");

lazy_static! {
    static ref REGEX_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    static ref REGEX_NAME: Regex = Regex::new(r"^[\p{L}\p{Zs}'-]+$").unwrap();
}

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

pub async fn register() -> Html<String> {
    Html(templ::Register {}.render().unwrap())
}

pub async fn register_post(
    session: Session,
    ValidatedForm(form): ValidatedForm<RegisterInfo>,
) -> Response {
    let mut user = User::new();
    user.username = form.username;
    user.firstname = form.firstname;
    user.lastname = form.lastname;
    user.passhash = bcrypt::hash(form.password, 4).unwrap();

    if user::is_username_used(&user.username).unwrap() {
        return "Username already used".into_response();
    }

    let id = user::register(user).unwrap();

    session.insert(SESSION_ID_KEY, id).await.unwrap();
    Redirect::to("/").into_response()
}
