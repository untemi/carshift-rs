use crate::{error::ServerResult, middlewares::OptionalLogginProps, templ};
use axum::{Extension, response::Response};
use std::ops::Deref;

pub async fn navbar(Extension(user): OptionalLogginProps) -> ServerResult<Response> {
    templ::render(templ::Navbar { user: user.deref() })
}
