pub mod app;
pub mod model;
pub mod components;
pub mod pages;
pub mod utils;
pub mod state;
// Re-export types and functions for easy access
pub use app::{shell, App};
pub use model::{Project, JDArea, JDCategory, get_all_areas, get_all_categories, find_category_by_id};
pub use components::*;
pub use pages::*;
pub use utils::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
