use axum::extract::Path;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;

use csutils::error::*;
use mw::{LogginProps, OptionalLogginProps};
use std::ops::Deref;

pub async fn mine(Extension(user): LogginProps) -> ServerResult<Response> {
    templ::render(templ::Profile {
        user: &user,
        is_self: true,
        is_logged: true,
    })
}

pub async fn other(
    Path(username): Path<String>,
    Extension(logged_user): OptionalLogginProps,
) -> ServerResult<Response> {
    if let Some(user) = logged_user.deref()
        && user.username == username
    {
        return Ok(Redirect::to("/profile").into_response());
    }

    let Some(user) = db::user::fetch_one_by_username(username).await? else {
        return Ok("not found".into_response());
    };

    templ::render(templ::Profile {
        user: &user,
        is_self: false,
        is_logged: logged_user.is_some(),
    })
}
