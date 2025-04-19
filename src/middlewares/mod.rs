use axum::{
    Extension,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::{db::*, error::*};
use std::sync::Arc;
use tower_sessions::Session;

pub type LogginProps = Extension<Arc<User>>;

pub const SESSION_ID_KEY: &str = "user_id";

pub async fn ensure_guest(session: Session, req: Request, next: Next) -> ServerResult<Response> {
    if fetch_login(&session).await?.is_some() {
        return Ok(Redirect::to("/").into_response());
    }

    Ok(next.run(req).await)
}

pub async fn ensure_user(session: Session, mut req: Request, next: Next) -> ServerResult<Response> {
    let Some(user) = fetch_login(&session).await? else {
        return Ok(Redirect::to("/login").into_response());
    };

    let user_state = Arc::new(user);
    req.extensions_mut().insert(user_state);
    Ok(next.run(req).await)
}

pub async fn fetch_login(session: &Session) -> ServerResult<Option<User>> {
    let Some(id) = session
        .get::<u64>(SESSION_ID_KEY)
        .await
        .map_err(AnyError::new)?
    else {
        return Ok(None);
    };

    let Some(user) = user::fetch_one_by_id(id)? else {
        session.delete().await.map_err(AnyError::new)?;
        return Ok(None);
    };

    Ok(Some(user))
}
