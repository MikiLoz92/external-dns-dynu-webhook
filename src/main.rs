mod state;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use tokio::signal;
use crate::state::AppState;

#[tokio::main]
async fn main() {

    let app_state = AppState::new();

    let app = Router::new()
        .route("/", get(root))
        .with_state(app_state);


    println!("Axum is serving on port 80!");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();

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
