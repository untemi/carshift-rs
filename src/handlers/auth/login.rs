use axum::{
    Form,
    response::{Html, IntoResponse, Redirect, Response},
};

use crate::db::*;
use crate::templ;
use askama::Template;
use tower_sessions::Session;

use super::*;

pub async fn login() -> Html<String> {
    Html(templ::Login {}.render().unwrap())
}

pub async fn login_post(session: Session, Form(form): Form<LoginInfo>) -> Response {
    let Some(user) = user::fetch_one_by_username(&form.username).unwrap() else {
        return "User or Password is bad".into_response();
    };

    if !bcrypt::verify(&form.password, &user.passhash).unwrap() {
        return "User or Password is bad".into_response();
    }

    session.insert(SESSION_ID_KEY, user.id).await.unwrap();
    Redirect::to("/").into_response()
}
