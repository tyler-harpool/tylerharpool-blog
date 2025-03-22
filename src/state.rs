// src/state.rs
#![cfg(feature = "ssr")]

use sqlx::SqlitePool;
use std::sync::Arc;
use leptos::prelude::LeptosOptions;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions, // adjust type as needed
    pub pool: Arc<SqlitePool>,
}
