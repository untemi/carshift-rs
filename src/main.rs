use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};

mod db;
mod extractors;
mod handlers;
mod middlewares;
mod misc;
mod templ;

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
        let guest_routes = Router::new()
            .route("/login", get(handlers::auth::login))
            .route("/login", post(handlers::auth::login_post))
            .route("/register", get(handlers::auth::register))
            .route("/register", post(handlers::auth::register_post))
            .layer(from_fn(middlewares::ensure_guest));

        let user_routes = Router::new()
            .route("/logout", get(handlers::auth::logout))
            .route("/profile", get(handlers::profile))
            .layer(from_fn(middlewares::ensure_user));

        let other_routes = Router::new().route("/", get(handlers::home));

        let htmx = Router::new().route("/navbar", get(handlers::components::navbar));

        Router::new()
            .merge(guest_routes)
            .merge(user_routes)
            .nest("/htmx", htmx)
            .merge(other_routes)
            .layer(session_layer)
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
