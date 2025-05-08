use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{get, get_service, post};

use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

mod db;
mod error;
mod handlers;
mod log;
mod middlewares;
mod misc;
mod templ;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let session_layer = {
        let manager = r2d2_sqlite::SqliteConnectionManager::file("session.db");
        let pool = r2d2::Pool::new(manager)?;
        let store = tower_sessions_r2d2_sqlite_store::SqliteStore::new(pool);
        store.migrate()?;

        tower_sessions::SessionManagerLayer::new(store).with_secure(false)
    };

    let router = {
        let tokenized = {
            let guest = Router::new()
                .route("/login", get(handlers::user::login))
                .route("/login", post(handlers::user::login_post))
                .route("/register", get(handlers::user::register))
                .route("/register", post(handlers::user::register_post))
                .layer(from_fn(middlewares::ensure_guest));

            let user = Router::new()
                // Settings
                .route("/settings", get(handlers::user::settings))
                .route("/settings/profile", get(handlers::user::profile))
                .route("/settings/profile", post(handlers::user::profile_post))
                .route("/settings/account", get(handlers::user::account))
                .route("/settings/account", post(handlers::user::account_post))
                .route("/settings/picture", post(handlers::user::upload_picture))
                // Misc
                .route("/logout", get(handlers::user::logout))
                .route("/profile", get(handlers::profile::mine))
                .layer(from_fn(middlewares::ensure_user));

            let optional = Router::new()
                .route("/htmx/navbar-info", get(handlers::blocks::navbar))
                .route("/user/{username}", get(handlers::profile::other))
                .layer(from_fn(middlewares::optional_user));

            Router::new()
                .merge(guest)
                .merge(user)
                .merge(optional)
                .layer(session_layer)
        };

        // file serving
        let file_serve = Router::new()
            .nest_service("/static", get_service(ServeDir::new("static")))
            .nest_service("/pictures", get_service(ServeDir::new("pictures")))
            .nest_service(
                "/favicon.ico",
                get_service(ServeFile::new("static/favicon.ico")),
            );

        let unconditional_pages = Router::new()
            .route("/", get(handlers::home))
            .route("/htmx/search-users", post(handlers::search::users::find))
            .route("/htmx/search-cars", post(handlers::search::cars::find))
            .route("/htmx/alert", post(handlers::blocks::alert))
            .route("/search-users", get(handlers::search::users::page))
            .route("/search-cars", get(handlers::search::cars::page));

        Router::new()
            .merge(tokenized)
            .merge(unconditional_pages)
            .merge(file_serve)
            .layer(from_fn(log::log_request))
    };

    println!("SERVER: running on 127.0.0.1:8000");
    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:8000").await?,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
