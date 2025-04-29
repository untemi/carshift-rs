use crate::{db::*, error::*, ico, ico_mini};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

// the actual mod.rs lmao
#[allow(dead_code)]
pub enum AlertLevel {
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Template)]
#[template(path = "components/alert.html")]
pub struct Alert {
    pub level: AlertLevel,
    pub message: String,
}

pub struct Tab {
    pub name: &'static str,
    pub link: &'static str,
}

pub fn render<T: Template>(template: T) -> ServerResult<Response> {
    let template = template.render().map_err(AnyError::new)?;
    let res = Html(template).into_response();
    Ok(res)
}

// mental ilness
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
#[template(path = "components/navbar-info.html")]
pub struct Navbar<'a> {
    pub user: &'a Option<User>,
}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct Profile<'a> {
    pub user: &'a User,

    pub is_self: bool,
    pub is_logged: bool,
}

// settings
#[derive(Template)]
#[template(path = "settings.html")]
pub struct Settings {
    pub tabs: &'static [Tab],
}

#[derive(Template)]
#[template(path = "settings/profile.html")]
pub struct SettingsProfile<'a> {
    pub user: &'a User,
}

#[derive(Template)]
#[template(path = "settings/account.html")]
pub struct SettingsAccount<'a> {
    pub user: &'a User,
}
