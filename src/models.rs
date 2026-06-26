use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// A generic "item" resource — replace with your real domain type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Stable, globally-unique identifier.
    pub id: Uuid,
    /// Human-readable label.
    pub name: String,
    /// Optional extended description.
    pub description: Option<String>,
    /// UTC creation timestamp.
    pub created_at: DateTime<Utc>,
    /// UTC last-update timestamp.
    pub updated_at: DateTime<Utc>,
}

impl Item {
    /// Create a new `Item` with the current timestamp.
    pub fn new(name: impl Into<String>, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Payload for creating a new item.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateItemRequest {
    /// Display name – 1–100 characters.
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    /// Optional description – up to 500 characters.
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

/// Payload for (partially) updating an item.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateItemRequest {
    /// New display name if provided.
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    /// New description if provided (`null` clears it).
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

/// Paginated list response wrapper.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    /// The items on this page.
    pub data: Vec<T>,
    /// Total number of items across all pages.
    pub total: usize,
    /// 1-based page number.
    pub page: usize,
    /// Number of items per page.
    pub per_page: usize,
}

/// Query parameters for paginated list endpoints.
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    /// Page number (default 1).
    #[serde(default = "default_page")]
    pub page: usize,
    /// Items per page (default 20, max 100).
    #[serde(default = "default_per_page")]
    pub per_page: usize,
}

fn default_page() -> usize {
    1
}

fn default_per_page() -> usize {
    20
}