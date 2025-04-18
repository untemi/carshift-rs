use axum::{
    Router,
    middleware::from_fn,
    routing::{get, get_service, post},
};
use tower_http::services::ServeDir;

mod db;
mod extractors;
mod handlers;
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
                .route("/login", get(handlers::auth::login))
                .route("/login", post(handlers::auth::login_post))
                .route("/register", get(handlers::auth::register))
                .route("/register", post(handlers::auth::register_post))
                .layer(from_fn(middlewares::ensure_guest));

            let user = Router::new()
                .route("/logout", get(handlers::auth::logout))
                .route("/profile", get(handlers::profile))
                .layer(from_fn(middlewares::ensure_user));

            let htmx = Router::new().route("/navbar", get(handlers::components::navbar));

            Router::new()
                .merge(guest)
                .merge(user)
                .nest("/htmx", htmx)
                .layer(session_layer)
        };

        let file_serve =
            Router::new().nest_service("/static", get_service(ServeDir::new("static"))); // static files

        let pages = Router::new().route("/", get(handlers::home));
        Router::new()
            .merge(tokenized)
            .merge(pages)
            .merge(file_serve)
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
