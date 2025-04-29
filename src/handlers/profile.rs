use axum::{
    Extension,
    extract::Path,
    response::{IntoResponse, Redirect, Response},
};

use crate::{
    db,
    error::*,
    middlewares::{LogginProps, OptionalLogginProps},
    templ,
};

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
    let Some(user) = db::user::fetch_one_by_username(&username)? else {
        return Ok("not found".into_response());
    };

    if let Some(user) = logged_user.deref() {
        if user.username == username {
            return Ok(Redirect::to("/profile").into_response());
        }
    }

    templ::render(templ::Profile {
        user: &user,
        is_self: false,
        is_logged: logged_user.is_some(),
    })
}
