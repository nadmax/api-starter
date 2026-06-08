use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use uuid::Uuid;

use crate::{config::AppConfig, models::Item};

/// Shared, thread-safe in-memory item store.
///
/// Replace with a database pool (e.g. `sqlx::PgPool`) when adding persistence.
pub type ItemStore = Arc<RwLock<HashMap<Uuid, Item>>>;

/// Inner state, held behind an [`Arc`] so it can be cheaply cloned across tasks.
#[derive(Debug)]
struct Inner {
    config: AppConfig,
    /// In-memory store — swap out for a real DB pool as the app grows.
    pub items: ItemStore,
}

/// Cheaply-cloneable handle to shared application state.
///
/// Add fields (database pools, caches, …) to [`Inner`] as your app grows.
#[derive(Clone, Debug)]
pub struct AppState(Arc<Inner>);

impl AppState {
    /// Create application state from the loaded config.
    pub fn new(config: AppConfig) -> Self {
        Self(Arc::new(Inner {
            config,
            items: Arc::new(RwLock::new(HashMap::new())),
        }))
    }

    /// Borrow the application configuration.
    pub fn config(&self) -> &AppConfig {
        &self.0.config
    }

    /// Borrow the item store handle.
    pub fn items(&self) -> &ItemStore {
        &self.0.items
    }
}