pub mod app;
pub mod model;

// Re-export types and functions for easy access
pub use app::*;
pub use model::*;
pub use model::{
    JDArea,
    JDCategory,
    JDId,
    get_all_areas,
    get_all_categories,
    find_category_by_id,
    find_area_by_id,
    get_categories_for_area
};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
