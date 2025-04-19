use axum::{
    Extension,
    response::{Html, IntoResponse, Response},
};

use crate::middlewares::LogginProps;
use crate::templ;
use askama::Template;

pub mod components;
pub mod user;

pub async fn home() -> Response {
    Html(templ::Home {}.render().unwrap()).into_response()
}

pub async fn profile(Extension(user): LogginProps) -> Html<String> {
    Html(templ::Profile { user: &user }.render().unwrap())
}
