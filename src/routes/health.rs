use axum::http::StatusCode;

/// Returns `200 OK` when the process is alive and accepting traffic.
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}