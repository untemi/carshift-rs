use crate::db::*;
use crate::middlewares::fetch_login;
use crate::templ;
use askama::Template;
use axum::response::Html;
use tower_sessions::Session;

pub async fn navbar(session: Session) -> Html<String> {
    let Some(user) = fetch_login(&session).await else {
        let template = templ::Navbar {
            user: &User::new(),
            logged: false,
        };

        return Html(template.render().unwrap());
    };

    let template = templ::Navbar {
        user: &user,
        logged: true,
    };

    Html(template.render().unwrap())
}
