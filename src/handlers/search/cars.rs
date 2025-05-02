use crate::error::ServerResult;
use crate::templ::{self, SearchCars};
use axum::response::Response;

pub async fn page() -> ServerResult<Response> {
    templ::render(SearchCars {})
}
