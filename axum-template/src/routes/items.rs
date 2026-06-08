use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::ApiError,
    models::{CreateItemRequest, Item, PaginatedResponse, PaginationQuery, UpdateItemRequest},
    state::AppState,
};

// ─── Router ──────────────────────────────────────────────────────────────────

/// Attach all item routes.
///
/// The store is owned by [`AppState`], so handlers extract it via
/// `State<AppState>` — no manual closure captures or extra parameters.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_items).post(create_item))
        .route("/:id", get(get_item).put(update_item).delete(delete_item))
}

// ─── Handlers ────────────────────────────────────────────────────────────────

/// `GET /api/v1/items` — paginated list of items.
async fn list_items(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let per_page = pagination.per_page.min(100);
    let page = pagination.page.max(1);

    let items: Vec<Item> = state
        .items()
        .read()
        .map_err(|e| ApiError::internal(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?
        .values()
        .cloned()
        .collect();

    let total = items.len();
    let skip = (page - 1) * per_page;
    let data = items.into_iter().skip(skip).take(per_page).collect();

    Ok(Json(PaginatedResponse { data, total, page, per_page }))
}

/// `POST /api/v1/items` — create a new item.
async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let item = Item::new(payload.name, payload.description);

    state
        .items()
        .write()
        .map_err(|e| ApiError::internal(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?
        .insert(item.id, item.clone());

    Ok((StatusCode::CREATED, Json(item)))
}

/// `GET /api/v1/items/:id` — fetch a single item by ID.
async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let guard = state
        .items()
        .read()
        .map_err(|e| ApiError::internal(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    let item = guard.get(&id).cloned().ok_or(ApiError::NotFound)?;
    Ok(Json(item))
}

/// `PUT /api/v1/items/:id` — partially update an item.
async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<impl IntoResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let mut guard = state
        .items()
        .write()
        .map_err(|e| ApiError::internal(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    let item = guard.get_mut(&id).ok_or(ApiError::NotFound)?;

    if let Some(name) = payload.name {
        item.name = name;
    }
    if let Some(description) = payload.description {
        item.description = Some(description);
    }
    item.updated_at = chrono::Utc::now();

    Ok(Json(item.clone()))
}

/// `DELETE /api/v1/items/:id` — remove an item.
async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let removed = state
        .items()
        .write()
        .map_err(|e| ApiError::internal(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?
        .remove(&id);

    if removed.is_none() {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}