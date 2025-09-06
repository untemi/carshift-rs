use axum::{extract::Path, response::Response, Form};
use csutils::error::{AnyError, ServerResult};
use db::Sortings;
use serde::{Deserialize, Serialize};
use templ::UserCars;

#[derive(Deserialize, Serialize)]
pub struct FetchInfo {
    id: i64,
    sort: Sortings,
}

pub async fn user_cars(Path(page): Path<i64>, Form(form): Form<FetchInfo>) -> ServerResult<Response> {
    let cars = db::car::fetch_from_user(form.id, form.sort, page - 1, 10).await?;
    let car_count = db::car::count_from_user(form.id).await? as f64;
    let page_count = (car_count / 10.).ceil() as i64;

    templ::render(UserCars {
        hx_vals: serde_json::to_string(&form).map_err(AnyError::new)?,
        cars,
        page: page,
        page_count,
    })
}
