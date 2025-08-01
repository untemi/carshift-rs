use axum::middleware::from_fn;
use axum::routing::{get, get_service, post};
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    db::init().await?;
    let session_layer = {
        let store = db::build_session_store().await?;
        tower_sessions::SessionManagerLayer::new(store)
    };

    let router = {
        let tokenized = {
            let guest = Router::new()
                .route("/login", get(handlers::user::login))
                .route("/login", post(handlers::user::login_post))
                .route("/register", get(handlers::user::register))
                .route("/register", post(handlers::user::register_post))
                .layer(from_fn(mw::ensure_guest));

            let user = Router::new()
                // Settings
                .route("/settings", get(handlers::user::settings::page))
                .route("/settings/profile", get(handlers::user::settings::profile))
                .route("/settings/profile", post(handlers::user::settings::profile_post))
                .route("/settings/account", get(handlers::user::settings::account))
                .route("/settings/account", post(handlers::user::settings::account_post))
                .route("/settings/picture", post(handlers::user::upload_picture))
                // Misc
                .route("/logout", get(handlers::user::logout))
                .route("/profile", get(handlers::user::display::mine))
                .layer(from_fn(mw::ensure_user));

            let optional = Router::new()
                .route("/htmx/navbar-info", get(handlers::blocks::navbar))
                .route("/user/{username}", get(handlers::user::display::other))
                .layer(from_fn(mw::optional_user));

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
            .nest_service("/favicon.ico", get_service(ServeFile::new("static/favicon.ico")));

        let unconditional_pages = Router::new()
            .route("/", get(handlers::home))
            .route("/car/{id}", get(handlers::car::display))
            // Search
            .route("/search-users", get(handlers::search::users::page))
            .route("/search-cars", get(handlers::search::cars::page))
            // Htmx
            .route("/htmx/search-users", post(handlers::search::users::find))
            .route("/htmx/search-cars", post(handlers::search::cars::find))
            .route("/htmx/alert", post(handlers::blocks::alert));

        Router::new()
            .merge(tokenized)
            .merge(unconditional_pages)
            .merge(file_serve)
            .layer(from_fn(csutils::log::log_request))
    };

    println!("SERVER: running on http://127.0.0.1:8000");
    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:8000").await?,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
