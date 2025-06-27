use crate::error::*;
use crate::templ;
use axum::response::{IntoResponse, Response};

pub mod blocks;
pub mod car;
pub mod profile;
pub mod search;
pub mod user;

pub async fn home() -> ServerResult<Response> {
    templ::render(templ::Home {})
}

pub fn hx_redirect(uri: &str) -> Response {
    ([("HX-Redirect", uri)]).into_response()
}
