use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

/// All errors that can be returned by API handlers.
#[derive(Debug, Error)]
pub enum ApiError {
    /// A requested resource does not exist.
    #[error("resource not found")]
    NotFound,

    /// The request payload failed validation.
    #[error("validation error: {0}")]
    Validation(String),

    /// The caller is not authenticated.
    #[error("unauthorized")]
    Unauthorized,

    /// The caller lacks permission for the requested action.
    #[error("forbidden")]
    Forbidden,

    /// A conflict prevents the operation (e.g. duplicate key).
    #[error("conflict: {0}")]
    Conflict(String),

    /// An unexpected internal error occurred.
    ///
    /// The `id` is logged server-side; only a generic message is sent to the
    /// caller to avoid leaking implementation details.
    #[error("internal server error (id={id})")]
    Internal {
        id: Uuid,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl ApiError {
    /// Create an [`ApiError::Internal`] from any error, logging it and
    /// assigning a correlation ID that is returned to the caller.
    pub fn internal(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        let id = Uuid::new_v4();
        tracing::error!(error.id = %id, error = %source, "internal server error");
        Self::Internal {
            id,
            source: Box::new(source),
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            Self::NotFound => "NOT_FOUND",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::Conflict(_) => "CONFLICT",
            Self::Internal { .. } => "INTERNAL_SERVER_ERROR",
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status();
        let body = match &self {
            Self::Internal { id, .. } => json!({
                "error": {
                    "code": self.error_code(),
                    "message": "An unexpected error occurred. Please try again later.",
                    "request_id": id,
                }
            }),
            _ => json!({
                "error": {
                    "code": self.error_code(),
                    "message": self.to_string(),
                }
            }),
        };
        (status, Json(body)).into_response()
    }
}