pub mod format;
pub mod markdown;
pub mod jd_content_loader;
pub mod directory_scanner; // New module

pub use format::format_date;
pub use markdown::markdown_to_html;
pub use jd_content_loader::{load_markdown_files, markdown_to_projects};
pub use directory_scanner::{get_cached_areas, get_cached_categories}; // Export cached functions
