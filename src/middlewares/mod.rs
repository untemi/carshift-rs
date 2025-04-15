use std::sync::Arc;

use axum::{
    Extension,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::db::*;
use tower_sessions::Session;

pub type LogginProps = Extension<Arc<User>>;

pub const SESSION_ID_KEY: &str = "user_id";

pub async fn ensure_guest(session: Session, req: Request, next: Next) -> Response {
    if fetch_login(&session).await.is_some() {
        return Redirect::to("/").into_response();
    }

    next.run(req).await
}

pub async fn ensure_user(session: Session, mut req: Request, next: Next) -> Response {
    let Some(user) = fetch_login(&session).await else {
        return Redirect::to("/login").into_response();
    };

    let user_state = Arc::new(user);

    req.extensions_mut().insert(user_state);
    next.run(req).await
}

pub async fn fetch_login(session: &Session) -> Option<User> {
    let id = session.get::<u64>(SESSION_ID_KEY).await.unwrap()?;
    let Some(user) = user::fetch_one_by_id(id).unwrap() else {
        session.delete().await.unwrap();
        return None;
    };

    Some(user)
}
