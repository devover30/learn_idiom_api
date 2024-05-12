mod error;
mod idiom_resource;
mod models;
mod tests;
use axum::routing::{post, put};
use axum::Router;
use error::AppError;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, process};
use tokio::net::TcpListener;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "learn_idiom_v1=debug,tower_http=debug,axum=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(err) => {
            tracing::error!("env var error returned: {}", err);
            tracing::error!("DATABASE_URL env variable required");
            process::exit(1);
        }
    };

    let pool = match get_db_pool_conn(database_url).await {
        Ok(conn) => conn,
        Err(err) => {
            tracing::error!("error returned: {}", err);
            tracing::error!("Error connecting to database");
            process::exit(1);
        }
    };

    run_migrations(pool.clone()).await;

    let app = Router::new().nest("/api/v1", app(pool));

    let app = app.fallback(handler_404);

    let addr = SocketAddr::from(([0, 0, 0, 0], 11800));
    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind(&addr).await.unwrap();

    tracing::debug!("listening on {}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

pub fn app(db: PgPool) -> Router {
    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        // Here we setup the routes. Note: No macros
        .route(
            "/idioms",
            post(idiom_resource::controller::get_idiom_by_user),
        )
        .route(
            "/idioms/:idiom_id",
            put(idiom_resource::controller::update_idiom_read_action),
        )
        .with_state(Arc::new(models::AppState { db }))
        .layer(cors)
        // Using tower to add tracing layer
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
}

pub async fn get_db_pool_conn(connection: String) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection)
        .await
}

pub async fn run_migrations(pool: PgPool) {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Error Running Migrations")
}

async fn handler_404() -> AppError {
    AppError::NotFoundError
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
