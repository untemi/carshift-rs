use crate::{db::*, error::ServerResult, middlewares::fetch_login, templ};
use axum::response::Response;
use tower_sessions::Session;

pub async fn navbar(session: Session) -> ServerResult<Response> {
    let Some(user) = fetch_login(&session).await? else {
        let template = templ::Navbar {
            user: &User::default(),
            logged: false,
        };

        return templ::render(template);
    };

    let template = templ::Navbar {
        user: &user,
        logged: true,
    };

    templ::render(template)
}
