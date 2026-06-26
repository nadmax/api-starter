//! Router construction — all routes and middleware layers are wired here.

use std::time::Duration;

use axum::{
    Router, http::{Method, StatusCode, header}, routing::get
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::state::AppState;

mod health;
mod items;

/// Build the complete [`Router`] with all routes and middleware attached.
pub fn build_router(state: AppState) -> Router {
    let timeout = Duration::from_secs(state.config().server.request_timeout_secs);

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, timeout))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_origin(Any),
        );

    Router::new()
        .route("/health", get(health::health_check))
        .nest("/api/v1", api_v1_router())
        .with_state(state)
        .layer(middleware)
}

/// All `/api/v1/…` routes.
fn api_v1_router() -> Router<AppState> {
    Router::new().nest("/items", items::router())
}