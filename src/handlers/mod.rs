use crate::{error::*, templ};
use axum::response::{IntoResponse, Response};

pub mod components;
pub mod profile;
pub mod user;

pub async fn home() -> ServerResult<Response> {
    templ::render(templ::Home {})
}

pub fn hx_redirect(uri: &str) -> Response {
    ([("HX-Redirect", uri)]).into_response()
}
