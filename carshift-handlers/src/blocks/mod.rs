use csutils::error::ServerResult;
use mw::OptionalLogginProps;

use axum::response::Response;
use axum::{Extension, Form};
use serde::Deserialize;

pub async fn navbar(Extension(user): OptionalLogginProps) -> ServerResult<Response> {
    templ::render(templ::Navbar { user: &user })
}

#[derive(Deserialize)]
pub struct AlertParams {
    message: String,
    level: templ::AlertLevel,
}

pub async fn alert(Form(form): Form<AlertParams>) -> ServerResult<Response> {
    templ::render(templ::Alert {
        level: form.level,
        message: form.message,
    })
}
