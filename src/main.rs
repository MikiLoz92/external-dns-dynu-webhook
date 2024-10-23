mod state;
mod http;

use crate::http::{adjust_endpoints, retrieve_dns_records, retrieve_domain_filter, apply_changes};
use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {

    let logging_layer = tracing_subscriber::fmt::layer()
        .event_format(tracing_subscriber::fmt::format())
        .with_line_number(true)
        .with_writer(std::io::stdout.with_max_level(tracing::Level::TRACE));

    tracing_subscriber::registry()
        .with(logging_layer)
        .init();

    let app_state = AppState::new(
        reqwest::Client::new(),
        std::env::var("DYNU_API_KEY").unwrap(),
        std::env::var("DYNU_DOMAIN_NAMES").unwrap().split(",").map(|s| s.to_owned()).collect(),
        std::env::var("DYNU_GROUP_NAME").ok(),
    );

    let app = Router::new()
        .route("/", get(retrieve_domain_filter))
        .route("/records", get(retrieve_dns_records))
        .route("/records", post(apply_changes))
        .route("/adjustendpoints", post(adjust_endpoints))
        .with_state(app_state);

    tracing::info!("Axum is serving on port 8888!");

    let health_probes_listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let app_listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();

    let hp_task_handle = tokio::spawn(async move {
        axum::serve(
            health_probes_listener,
            Router::new().route("/healthz", get(root)).into_make_service_with_connect_info::<SocketAddr>()
        ).await.unwrap();
    });

    axum::serve(app_listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
            tracing::info!("Received SIGTERM signal! Propagating termination...");
        })
        .await.unwrap();

    tracing::info!("Shutting down!");
    hp_task_handle.abort();
    tracing::info!("Goodbye!");
}

async fn root() {} // default empty 200 response for readiness & liveness checks
