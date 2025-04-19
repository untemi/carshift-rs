use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

use crate::db::*;
use crate::error::*;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {}

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login {}

#[derive(Template)]
#[template(path = "register.html")]
pub struct Register {}

#[derive(Template)]
#[template(path = "components/alert.html")]
pub struct Alert {
    pub level: AlertLevel,
    pub message: String,
}

#[derive(Template)]
#[template(path = "components/navbar.html")]
pub struct Navbar<'a> {
    pub logged: bool,
    pub user: &'a User,
}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct Profile<'a> {
    pub user: &'a User,
}

pub fn render<T: Template>(template: T) -> ServerResult<Response> {
    let template = template.render().map_err(AnyError::new)?;
    let res = Html(template).into_response();
    Ok(res)
}

pub enum AlertLevel {
    Success,
    Info,
    Warning,
    Error,
}
