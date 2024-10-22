mod state;
mod http;

use crate::http::{adjust_endpoints, retrieve_dns_records, retrieve_domain_filter};
use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() {

    let app_state = AppState::new(
        reqwest::Client::new(),
        std::env::var("DYNU_API_KEY").unwrap(),
        std::env::var("DYNU_DOMAIN_NAMES").unwrap().split(",").map(|s| s.to_owned()).collect(),
    );

    let app = Router::new()
        .route("/", get(retrieve_domain_filter))
        .route("/healthz", get(root))
        .route("/records", get(retrieve_dns_records))
        .route("/adjustendpoints", post(adjust_endpoints))
        .with_state(app_state);

    println!("Axum is serving on port 8888!");

    let health_probes_listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let app_listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();

    axum::serve(
        health_probes_listener,
        Router::new().route("/healthz", get(root)).into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();

    axum::serve(app_listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
            tracing::info!("Received SIGTERM signal! Propagating termination...");
        })
        .await.unwrap();

    tracing::info!("Shutting down!")
}

async fn root() {} // default empty 200 response for readiness & liveness checks
