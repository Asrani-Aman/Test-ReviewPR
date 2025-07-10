use axum::{routing::{get, post}, Router};
use crate::{handlers, AppState}; // Import the handlers module

/// Creates the main application router.
pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/api/health", get(handlers::health_check))
        .route("/api/webhooks/github", post(handlers::github_webhook_handler))
        .route("/api/prs", get(handlers::get_reviewed_prs))
}
