use axum::{
    extract::State,
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod database;
mod handlers;
mod models;
mod services;
mod websocket;
mod itdoc;
mod files;
mod notifications;
mod integrations;

pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub ws_manager: websocket::WsManager,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env()?;
    let db_pool = database::create_pool(&config.database_url).await?;
    
    database::migrate(&db_pool).await?;

    let ws_manager = websocket::WsManager::new();
    let app_state = Arc::new(AppState { db_pool, ws_manager });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "GhostHub MSP Platform API v1.0.0" }))
        .route("/health", get(handlers::health_check))
        .route("/api/v1/dashboard", get(handlers::dashboard_stats))
        .nest("/api/v1/auth", auth::auth_routes())
        .nest("/api/v1/clients", handlers::client_routes())
        .nest("/api/v1/tickets", handlers::ticket_routes())
        .nest("/api/v1/assets", handlers::asset_routes())
        .nest("/api/v1/invoices", handlers::invoice_routes())
        .nest("/api/v1/time", handlers::time_tracking_routes())
        .nest("/api/v1/projects", handlers::project_routes())
        .nest("/api/v1/kb", handlers::knowledge_base_routes())
        .nest("/api/v1/portal", handlers::portal_routes())
        .nest("/api/v1/itdoc", itdoc::itdoc_routes())
        .nest("/api/v1/files", files::file_routes())
        .nest("/api/v1/notifications", notifications::notification_routes())
        .nest("/api/v1/integrations", integrations::integration_routes())
        .nest("/api/v1/users", handlers::user_routes())
        .route("/ws", get(websocket::websocket_handler))
        .layer(ServiceBuilder::new().layer(cors))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.server_addr).await?;
    tracing::info!("Server running on {}", config.server_addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}