mod state;

use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use commands::{CommandRequest, CommandResponse};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::services::{ServeDir, ServeFile};

async fn command(
    State(state): State<Arc<AppState>>,
    Json(cmd): Json<CommandRequest>,
) -> (StatusCode, Json<CommandResponse>) {
    match state.command(cmd.name, cmd.req).await {
        Ok(res) => (StatusCode::OK, Json(CommandResponse { res: Ok(res) })),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(CommandResponse { res: Err(err) }),
        ),
    }
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

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/command", post(command))
        .fallback_service(
            ServeDir::new("../frontend/dist")
                .not_found_service(ServeFile::new("../frontend/dist/index.html")),
        )
        .with_state(Arc::new(AppState::new()));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
