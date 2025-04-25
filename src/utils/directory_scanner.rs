use std::fs;
use std::path::Path;
use crate::model::{JDArea, JDCategory};
use leptos::logging::log;
use std::sync::{Mutex, Once};

// Regex parsing
use regex::Regex;

pub fn scan_content_structure(root_dir: &str) -> (Vec<JDArea>, Vec<JDCategory>) {
    let mut areas = Vec::new();
    let mut categories = Vec::new();
    let root = Path::new(root_dir);

    if !root.exists() || !root.is_dir() {
        log!("Content directory not found: {}", root_dir);
        return (areas, categories);
    }

    log!("Scanning content structure in: {}", root_dir);

    let area_re = Regex::new(r"^(\d{1,2})-(\d{1,2})\s+(.*)").unwrap();

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() { continue; }

            // Check for IGNORE file
            if path.join("IGNORE").exists() {
                log!("Skipping area due to IGNORE file: {:?}", path);
                continue;
            }

            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if let Some(caps) = area_re.captures(dir_name) {
                    let start_area: u8 = caps[1].parse().unwrap_or(0);
                    let _end_area: u8 = caps[2].parse().unwrap_or(0);
                    let area_name = caps[3].trim().to_string();

                    if start_area % 10 == 0 {
                        log!("Found area: {} ({})", area_name, start_area);

                        areas.push(JDArea {
                            id: start_area,
                            name: area_name,
                            description: read_description(&path),
                        });

                        scan_categories(&path, start_area, &mut categories);
                    }
                }
            }
        }
    } else {
        log!("Failed to read content root directory: {}", root_dir);
    }

    areas.sort_by_key(|a| a.id);
    categories.sort_by_key(|c| c.id);

    log!("Found {} areas and {} categories", areas.len(), categories.len());
    (areas, categories)
}

fn read_description(path: &Path) -> String {
    for filename in &["README.md", "index.md"] {
        let desc_path = path.join(filename);
        if desc_path.exists() {
            if let Ok(content) = fs::read_to_string(&desc_path) {
                let mut paragraph = String::new();
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with('#') || trimmed.is_empty() {
                        if !paragraph.is_empty() {
                            break;
                        }
                        continue;
                    }
                    paragraph.push_str(trimmed);
                    paragraph.push(' ');
                }
                if !paragraph.is_empty() {
                    return paragraph.trim().to_string();
                }
            } else {
                log!("Failed to read file: {:?}", desc_path);
            }
        }
    }

    "No description available".to_string()
}

fn scan_categories(area_path: &Path, area_id: u8, categories: &mut Vec<JDCategory>) {
    let cat_re = Regex::new(r"^(\d{1,2})\s+(.*)").unwrap();

    if let Ok(entries) = fs::read_dir(area_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() { continue; }

            // Skip category if it has an IGNORE file
            if path.join("IGNORE").exists() {
                log!("Skipping category due to IGNORE file: {:?}", path);
                continue;
            }

            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if let Some(caps) = cat_re.captures(dir_name) {
                    if let Ok(cat_id) = caps[1].parse::<u8>() {
                        if cat_id / 10 == area_id / 10 && cat_id % 10 != 0 {
                            let cat_name = caps[2].trim().to_string();
                            log!("Found category: {} ({}) in area {}", cat_name, cat_id, area_id);

                            categories.push(JDCategory {
                                id: cat_id,
                                area_id,
                                name: cat_name,
                                description: read_description(&path),
                            });
                        }
                    }
                }
            }
        }
    } else {
        log!("Failed to read directory: {:?}", area_path);
    }
}


// Caching
struct CachedData {
    areas: Vec<JDArea>,
    categories: Vec<JDCategory>,
}

static INIT: Once = Once::new();
static CACHE: Mutex<Option<CachedData>> = Mutex::new(None);

pub fn get_cached_structure(force_refresh: bool) -> (Vec<JDArea>, Vec<JDCategory>) {
    if force_refresh {
        *CACHE.lock().unwrap() = None;
    }

    INIT.call_once(|| {
        refresh_cache();
    });

    let cache = CACHE.lock().unwrap();
    if let Some(ref data) = *cache {
        (data.areas.clone(), data.categories.clone())
    } else {
        let content_dir = option_env!("CONTENT_DIR").unwrap_or("content/blog");
        scan_content_structure(content_dir)
    }
}

fn refresh_cache() {
    let content_dir = option_env!("CONTENT_DIR").unwrap_or("content/blog");
    let (areas, categories) = scan_content_structure(content_dir);

    let mut cache = CACHE.lock().unwrap();
    *cache = Some(CachedData {
        areas,
        categories,
    });
}

pub fn get_cached_areas() -> Vec<JDArea> {
    get_cached_structure(false).0
}

pub fn get_cached_categories() -> Vec<JDCategory> {
    get_cached_structure(false).1
}
