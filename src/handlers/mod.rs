use crate::{error::*, middlewares::LogginProps, templ};
use axum::{
    Extension,
    response::{IntoResponse, Response},
};

pub mod components;
pub mod user;

pub async fn home() -> ServerResult<Response> {
    templ::render(templ::Home {})
}

pub async fn profile(Extension(user): LogginProps) -> ServerResult<Response> {
    templ::render(templ::Profile { user: &user })
}

pub fn hx_redirect(uri: &str) -> Response {
    ([("HX-Redirect", uri)]).into_response()
}
