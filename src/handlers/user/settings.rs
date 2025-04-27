use crate::{db, middlewares::LogginProps, misc::extractors::ValidatedForm, templ};
use axum::{Extension, response::Response};

use super::*;
use serde::Deserialize;
use std::ops::Deref;
use validator::Validate;

// them setting tabs
const TABS: [templ::Tab; 2] = [
    templ::Tab {
        name: "profile",
        link: "/settings/profile",
    },
    templ::Tab {
        name: "account",
        link: "/settings/account",
    },
];

// pages
pub async fn settings() -> ServerResult<Response> {
    templ::render(templ::Settings { tabs: &TABS })
}
pub async fn profile(Extension(user): LogginProps) -> ServerResult<Response> {
    templ::render(templ::SettingsProfile { user: &user })
}
pub async fn account(Extension(user): LogginProps) -> ServerResult<Response> {
    templ::render(templ::SettingsAccount { user: &user })
}

#[derive(Deserialize, Validate)]
pub struct ProfileInfo {
    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 2, max = 30, message = "should be between 4-28 characters"))]
    firstname: String,

    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 2, max = 30, message = "should be between 4-28 characters"))]
    lastname: Option<String>,
}
pub async fn profile_post(
    Extension(user): LogginProps,
    ValidatedForm(form): ValidatedForm<ProfileInfo>,
) -> ServerResult<Response> {
    // populating a new user struct with new & old data
    let updated_user = db::User {
        firstname: form.firstname,
        lastname: form.lastname,
        ..user.deref().clone()
    };

    // comparing old & new user
    if updated_user == *user.deref() {
        return templ::render(templ::Alert {
            level: templ::AlertLevel::Info,
            message: "nothing changed".to_string(),
        });
    };

    // to the db and horay
    db::user::update(updated_user)?;
    templ::render(templ::Alert {
        level: templ::AlertLevel::Success,
        message: "updated".to_string(),
    })
}

#[derive(Deserialize, Validate)]
pub struct AccountInfo {
    #[validate(regex(path = *REGEX_USERNAME, message = "illegal character used"))]
    #[validate(length(min = 3, max = 15, message = "should be between 4-28 characters"))]
    username: String,

    #[validate(email(message = "invalid email address"))]
    email: Option<String>,

    #[validate(regex(path = *REGEX_PHONE, message = "invalid phone number"))]
    phone: Option<String>,
}
pub async fn account_post(
    Extension(user): LogginProps,
    ValidatedForm(form): ValidatedForm<AccountInfo>,
) -> ServerResult<Response> {
    // is username used (like register)
    if user.username != form.username && db::user::is_username_used(&form.username)? {
        return Err(ServerError::Encode("Username already used"));
    }

    // populating a new user struct with new & old data
    let updated_user = db::User {
        username: form.username,
        phone: form.phone,
        email: form.email,
        ..user.deref().clone()
    };

    // comparing old & new user
    if updated_user == *user.deref() {
        return templ::render(templ::Alert {
            level: templ::AlertLevel::Info,
            message: "nothing changed".to_string(),
        });
    };

    // to the db and horay
    db::user::update(updated_user)?;
    templ::render(templ::Alert {
        level: templ::AlertLevel::Success,
        message: "updated".to_string(),
    })
}
