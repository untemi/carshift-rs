use crate::db::user;
use crate::error::ServerResult;
use crate::handlers::user::REGEX_USERNAME;
use crate::misc::extractors::ValidatedForm;
use crate::templ::{self, SearchUsers};

use axum::response::Response;
use serde::Deserialize;
use validator::Validate;

pub async fn page() -> ServerResult<Response> {
    templ::render(SearchUsers {})
}

#[derive(Deserialize, Validate)]
pub struct SearchInfo {
    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 1, message = "empty search"))]
    input: String,
    page: Option<u64>,
}

pub async fn find(ValidatedForm(form): ValidatedForm<SearchInfo>) -> ServerResult<Response> {
    let users = user::find_many(&form.input, form.page.unwrap_or(0), 10)?;

    templ::render(templ::ResultUsers {
        users,
        input: form.input,
        next_page: form.page.unwrap_or(0) + 1,
    })
}
