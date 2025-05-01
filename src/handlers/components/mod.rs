use crate::error::ServerResult;
use crate::middlewares::OptionalLogginProps;
use crate::templ;

use axum::Extension;
use axum::response::Response;

pub async fn navbar(Extension(user): OptionalLogginProps) -> ServerResult<Response> {
    templ::render(templ::Navbar { user: &user })
}
