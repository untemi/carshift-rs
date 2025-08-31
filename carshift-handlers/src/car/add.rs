use axum::response::Response;
use csutils::error::ServerResult;

pub async fn page() -> ServerResult<Response> {
    templ::render(templ::AddCar {})
}
