use crate::{error::*, middlewares::LogginProps, templ};
use axum::{Extension, response::Response};

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

pub async fn settings() -> ServerResult<Response> {
    templ::render(templ::Settings { tabs: &TABS })
}

pub async fn profile(Extension(user): LogginProps) -> ServerResult<Response> {
    templ::render(templ::SettingsProfile { user: &user })
}

// pub async fn profile_set(Extension(user): LogginProps) -> ServerResult<Response> {
//     todo!()
// }

pub async fn account(Extension(user): LogginProps) -> ServerResult<Response> {
    templ::render(templ::SettingsAccount { user: &user })
}

// pub async fn account_set(Extension(user): LogginProps) -> ServerResult<Response> {
//     todo!()
// }
