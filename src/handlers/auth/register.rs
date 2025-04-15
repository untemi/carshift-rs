use axum::{
    Form,
    response::{Html, IntoResponse, Redirect, Response},
};

use super::*;
use crate::db::*;
use crate::templ;
use askama::Template;
use tower_sessions::Session;

pub async fn register() -> Html<String> {
    Html(templ::Register {}.render().unwrap())
}

pub async fn register_post(session: Session, Form(form): Form<LoginInfo>) -> Response {
    let mut user = User::new();
    user.username = form.username;
    user.passhash = bcrypt::hash(form.password, 4).unwrap();

    if user::is_username_used(&user.username).unwrap() {
        return "Username already used".into_response();
    }

    let id = user::register(user).unwrap();

    session.insert(SESSION_ID_KEY, id).await.unwrap();
    Redirect::to("/").into_response()
}
