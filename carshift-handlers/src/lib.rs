use axum::response::{IntoResponse, Response};
use csutils::error::ServerResult;

pub mod blocks;
pub mod car;
pub mod search;
pub mod user;

pub async fn home() -> ServerResult<Response> {
    templ::render(templ::Home {})
}

pub fn hx_redirect(uri: &str) -> Response {
    ([("HX-Redirect", uri)]).into_response()
}
