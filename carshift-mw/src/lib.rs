use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;

use csutils::error::*;
use db::*;
use std::sync::Arc;
use tower_sessions::Session;

pub type LogginProps = Extension<Arc<User>>;
pub type OptionalLogginProps = Extension<Arc<Option<User>>>;

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
    let ext = Arc::new(user);

    req.extensions_mut().insert(ext);
    Ok(next.run(req).await)
}

pub async fn optional_user(
    session: Session,
    mut req: Request,
    next: Next,
) -> ServerResult<Response> {
    let user = fetch_login(&session).await?;
    let ext = Arc::new(user);

    req.extensions_mut().insert(ext);
    Ok(next.run(req).await)
}

async fn fetch_login(session: &Session) -> ServerResult<Option<User>> {
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
