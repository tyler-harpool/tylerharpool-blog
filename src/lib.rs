pub mod app;
pub mod model;
pub mod components;
pub mod pages;
pub mod utils;

// Re-export types and functions for easy access
pub use app::*;
pub use model::*;
pub use components::*;
pub use pages::*;
pub use utils::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
