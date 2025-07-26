use csutils::error::{AnyError, ServerResult};
use csutils::{ico, ico_mini};

use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use db::db_ref::DbRef;
use db::*;
use serde::Deserialize;

pub fn render<T: Template>(template: T) -> ServerResult<Response> {
    let template = template.render().map_err(AnyError::new)?;
    let res = Html(template).into_response();
    Ok(res)
}

// the actual mod.rs lmao
#[allow(dead_code)]
#[derive(Deserialize)]
pub enum AlertLevel {
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Template)]
#[template(path = "hx-blocks/alert.html")]
pub struct Alert {
    pub level: AlertLevel,
    pub message: String,
}

pub struct Tab {
    pub name: &'static str,
    pub link: &'static str,
}

// search results
#[derive(Template)]
#[template(path = "hx-blocks/users.html")]
pub struct ResultUsers {
    pub input: String,
    pub next_page: i64,
    pub users: Box<[User]>,
}

#[derive(Template)]
#[template(path = "hx-blocks/cars.html")]
pub struct ResultCars {
    pub hx_vals: String,
    pub cars: Box<[Car]>,
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
#[template(path = "search-users.html")]
pub struct SearchUsers {}

#[derive(Template)]
#[template(path = "search-cars.html")]
pub struct SearchCars {}

#[derive(Template)]
#[template(path = "hx-blocks/navbar-info.html")]
pub struct Navbar<'a> {
    pub user: &'a Option<User>,
}

#[derive(Template)]
#[template(path = "car.html")]
pub struct DisplayCar<'a> {
    pub car: &'a Car,
}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct Profile<'a> {
    pub user: &'a User,

    pub is_self: bool,
    pub is_logged: bool,
}
