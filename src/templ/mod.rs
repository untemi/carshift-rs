use askama::Template;

use crate::db::*;

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
