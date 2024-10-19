mod state;
mod http;

use crate::http::retrieve_dns_records;
use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() {

    let app_state = AppState::new(
        reqwest::Client::new(),
        std::env::var("DYNU_API_KEY").unwrap(),
    );

    let app = Router::new()
        .route("/healthz", get(root))
        .route("/records", get(retrieve_dns_records))
        .with_state(app_state);

    println!("Axum is serving on port 8888!");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
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
