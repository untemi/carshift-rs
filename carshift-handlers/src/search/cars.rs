use axum::response::Response;
use axum::Form;
use chrono::NaiveDate;
use csutils::error::{AnyError, ServerError, ServerResult};
use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use templ::SearchCars;

pub async fn page() -> ServerResult<Response> {
    templ::render(SearchCars {})
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, IsEmpty)]
pub struct SearchInfo {
    input: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    district: Option<i64>,
    page: Option<i64>,
}

pub async fn find(Form(form): Form<SearchInfo>) -> ServerResult<Response> {
    // form.page is going to be empty when searching from input
    if form.is_empty() {
        return Err(ServerError::Encode("no input"));
    };

    if form.start_date.is_some() != form.end_date.is_some() {
        return Err(ServerError::Encode("invalid duration (both or none)"));
    }

    if let (Some(start_date), Some(end_date)) = (form.start_date, form.end_date)
        && start_date > end_date
    {
        return Err(ServerError::Encode("start date is greated than end date"));
    }

    let cars = db::car::find_many(
        form.input.clone(),
        form.start_date,
        form.end_date,
        form.district,
        form.page.unwrap_or(0),
        10,
    )
    .await?;

    let mut form = form;
    form.page = Some(form.page.unwrap_or(0) + 1);

    templ::render(templ::ResultCars {
        cars,
        hx_vals: serde_json::to_string(&form).map_err(AnyError::new)?,
    })
}
