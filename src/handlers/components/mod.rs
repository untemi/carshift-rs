use crate::{error::ServerResult, middlewares::OptionalLogginProps, templ};
use axum::{Extension, response::Response};

pub async fn navbar(Extension(user): OptionalLogginProps) -> ServerResult<Response> {
    templ::render(templ::Navbar { user: &user })
}
