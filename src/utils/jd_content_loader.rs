use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::model::{Project, JDArea, JDCategory};

pub struct FrontMatter {
    pub title: String,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub draft: bool,
    pub area_id: Option<u8>,
    pub category_id: Option<u8>,
    pub summary: Option<String>,
    pub related_articles: Vec<String>
}

// Extract Johnny Decimal ID from path or filename
pub fn extract_jd_info_from_path(path: &Path) -> (Option<u8>, Option<u8>, Option<u8>) {
    let path_str = path.to_string_lossy().to_string();
    let mut area_id: Option<u8> = None;
    let mut category_id: Option<u8> = None;
    let mut item_id: Option<u8> = None;

    // Try to extract from filename first (e.g., "21.01 Project Name.md")
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        // For patterns like "21.01 Project.md"
        if let Some(dot_pos) = file_name.find('.') {
            if dot_pos >= 2 {
                let cat_str = &file_name[..dot_pos];
                let item_str = &file_name[dot_pos+1..];

                if let Ok(cat) = cat_str.parse::<u8>() {
                    category_id = Some(cat);
                    area_id = Some((cat / 10) * 10);

                    // Try to parse item ID (the part after the dot)
                    if let Some(space_pos) = item_str.find(' ') {
                        if let Ok(item) = item_str[..space_pos].parse::<u8>() {
                            item_id = Some(item);
                        }
                    } else if let Ok(item) = item_str.parse::<u8>() {
                        item_id = Some(item);
                    }
                }
            }
        }
        // For patterns like "21 Project.md" (category without item)
        else if let Some(space_pos) = file_name.find(' ') {
            if space_pos >= 1 {
                if let Ok(cat) = file_name[..space_pos].parse::<u8>() {
                    category_id = Some(cat);
                    area_id = Some((cat / 10) * 10);
                }
            }
        }
    }

    // If category still not found, try from the directory structure
    if category_id.is_none() {
        // Split path into parts
        let path_parts: Vec<&str> = path_str.split(|c| c == '/' || c == '\\').collect();

        for part in path_parts {
            // Look for category format (e.g., "21 Active Projects")
            if let Some(space_pos) = part.find(' ') {
                if space_pos >= 1 && space_pos <= 3 {
                    if let Ok(cat) = part[..space_pos].parse::<u8>() {
                        if cat % 10 != 0 { // Should not be an area (multiple of 10)
                            category_id = Some(cat);
                            area_id = Some((cat / 10) * 10);
                            break;
                        }
                    }
                }
            }

            // Look for area format (e.g., "20-29 Work")
            if let Some(dash_pos) = part.find('-') {
                if dash_pos >= 1 && dash_pos <= 3 {
                    if let Ok(start_area) = part[..dash_pos].parse::<u8>() {
                        if start_area % 10 == 0 { // Should be a multiple of 10
                            area_id = Some(start_area);
                            break;
                        }
                    }
                }
            }
        }
    }



    (area_id, category_id, item_id)
}

pub fn load_markdown_files(root_dir: &str) -> Vec<(PathBuf, FrontMatter, String)> {
    let mut results = Vec::new();
    let root = Path::new(root_dir);

    if !root.exists() || !root.is_dir() {
        println!("Directory not found: {}", root_dir);
        return results;
    }

    process_directory(root, &mut results);

    println!("Loaded {} markdown files from {}", results.len(), root_dir);
    results
}

fn process_directory(dir: &Path, results: &mut Vec<(PathBuf, FrontMatter, String)>) {
    // Skip templates and .obsidian directories
    if let Some(dir_name) = dir.file_name().and_then(|n| n.to_str()) {
        if dir_name == ".obsidian" || dir_name == "03 Templates" {
            return;
        }
    }

    // Check for IGNORE file
    let ignore_path = dir.join("IGNORE");
    if ignore_path.exists() {
        println!("Skipping ignored directory: {}", dir.display());
        return;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Skip README.md files and other unwanted files
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Skip README.md files and template files
                    if file_name == "README.md" || file_name.contains("Template") || file_name == "index.md" {
                        continue;
                    }
                }
            }

            if path.is_dir() {
                process_directory(&path, results);
            } else if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let (front_matter, processed_content) = match parse_front_matter(&content) {
                        Some((fm, content)) => (fm, content),
                        None => {
                            // Generate front matter from content
                            let (title, first_para) = extract_title_and_summary(&content);
                            let (area_id, category_id, _) = extract_jd_info_from_path(&path);

                            let fm = FrontMatter {
                                title,
                                date: None,
                                tags: Vec::new(),
                                draft: false,
                                area_id,
                                category_id,
                                summary: Some(first_para),
                                related_articles: Vec::new(),
                            };
                            (fm, content)
                        }
                    };

                    results.push((path, front_matter, processed_content));
                }
            }
        }
    }
}


// Extract title and first paragraph for summary
fn extract_title_and_summary(content: &str) -> (String, String) {
    let mut title = String::from("Untitled");
    let mut first_para = String::new();

    // Find the title (first H1)
    for line in content.lines() {
        if line.starts_with("# ") {
            title = line[2..].trim().to_string();
            break;
        }
    }

    // Find the first paragraph that's not a heading
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            first_para = trimmed.to_string();
            break;
        }
    }

    // Limit summary length
    if first_para.len() > 150 {
        first_para = first_para[..150].to_string() + "...";
    }

    (title, first_para)
}

// Parse front matter from markdown content
fn parse_front_matter(content: &str) -> Option<(FrontMatter, String)> {
    let content = content.trim_start();

    // Check if the content starts with front matter delimiters
    let has_yaml = content.starts_with("---");
    let has_toml = content.starts_with("+++");

    if !has_yaml && !has_toml {
        println!("No front matter found, content starts with: {}", &content[..20.min(content.len())]);
        return None;
    }

    let delimiter = if has_yaml { "---" } else { "+++" };
    let parts: Vec<&str> = content.splitn(3, delimiter).collect();

    if parts.len() < 3 {
        println!("Front matter delimiters incomplete, found {} parts", parts.len());
        return None;
    }

    let front_matter_str = parts[1].trim();
    let main_content = parts[2].trim();

    // Parse front matter
    let mut title = String::new();
    let mut date = None;
    let mut tags = Vec::new();
    let mut draft = false;
    let mut area_id = None;
    let mut category_id = None;
    let mut summary = None;
    let mut related_articles = Vec::new();

    // Parse YAML or TOML front matter
    for line in front_matter_str.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        // Try to extract key-value pairs
        if let Some(idx) = line.find(':') {
            let key = line[..idx].trim();
            // Be more careful with value extraction to handle quoted strings properly
            let mut value = line[idx+1..].trim();
            if value.starts_with('"') && value.ends_with('"') {
                value = &value[1..value.len()-1];
            } else if value.starts_with('\'') && value.ends_with('\'') {
                value = &value[1..value.len()-1];
            }



            match key {
                "title" => title = value.to_string(),
                "date" => date = Some(value.to_string()),
                "draft" => draft = value.to_lowercase() == "true",
                "area_id" => area_id = value.parse::<u8>().ok(),
                "category_id" => category_id = value.parse::<u8>().ok(),
                "summary" => {

                    summary = Some(value.to_string())
                },
                "tags" => {
                    // Parse tags array
                    if value.starts_with('[') && value.ends_with(']') {
                        let tags_str = &value[1..value.len()-1];
                        tags = tags_str
                            .split(',')
                            .map(|s| {
                                let s = s.trim();
                                if (s.starts_with('"') && s.ends_with('"')) ||
                                   (s.starts_with('\'') && s.ends_with('\'')) {
                                    s[1..s.len()-1].to_string()
                                } else {
                                    s.to_string()
                                }
                            })
                            .collect();
                    }
                },
                "related_articles" => {
                    // Parse the related articles array
                    if value.starts_with('[') && value.ends_with(']') {
                        let articles_str = &value[1..value.len()-1];
                        related_articles = articles_str
                            .split(',')
                            .map(|s| {
                                let s = s.trim();
                                if (s.starts_with('"') && s.ends_with('"')) ||
                                   (s.starts_with('\'') && s.ends_with('\'')) {
                                    s[1..s.len()-1].to_string()
                                } else {
                                    s.to_string()
                                }
                            })
                            .collect();
                    }
                },
                _ => {}
            }
        }
    }

    // Extract title from the first heading if no title in front matter
    if title.is_empty() {
        for line in main_content.lines() {
            if line.starts_with("# ") {
                title = line[2..].trim().to_string();
                break;
            }
        }

        if title.is_empty() {
            title = "Untitled".to_string();
        }
    }

    // Create the FrontMatter struct with parsed values
    let front_matter = FrontMatter {
        title,
        date,
        tags,
        draft,
        area_id,
        category_id,
        summary,
        related_articles
    };

    // Debug the completed front matter


    Some((front_matter, main_content.to_string()))
}

// Convert markdown files to Projects with Johnny Decimal metadata
pub fn markdown_to_projects(root_dir: &str, areas: &[JDArea], categories: &[JDCategory]) -> Vec<Project> {
    let md_files = load_markdown_files(root_dir);

    md_files
        .into_iter()
        .filter(|(_, front_matter, _)| !front_matter.draft) // Skip drafts
        .map(|(path, front_matter, content)| {
            // Generate slug from title
            let slug = slug_from_title(&front_matter.title);

            // Extract Johnny Decimal ID directly from the path
            let (jd_area_id, jd_category_id, jd_item_id) = extract_jd_info_from_path(&path);

            // Create a JD identifier for display
            let jd_identifier = if let (Some(cat_id), Some(item_id)) = (jd_category_id, jd_item_id) {
                format!("{}.{:02}", cat_id, item_id)
            } else if let Some(cat_id) = jd_category_id {
                format!("{}", cat_id)
            } else {
                "".to_string()
            };

            // Additional fallback for jd_identifier from filename
            let jd_identifier = if jd_identifier.is_empty() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Try to find pattern like "21.01" in the filename
                    if file_name.len() >= 5 && file_name.chars().nth(2) == Some('.') {
                        let possible_id = &file_name[..5];
                        if possible_id.chars().filter(|c| c.is_digit(10) || *c == '.').count() == 5 {
                            possible_id.to_string()
                        } else {
                            "".to_string()
                        }
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                }
            } else {
                jd_identifier
            };

            // Parse the date if available
            let created_at = front_matter.date
                .as_ref()
                .and_then(|date_str| crate::utils::format::parse_date_string(date_str))
                .unwrap_or_else(|| SystemTime::now());

            // Find category based on the extracted JD info
            let category = if let Some(cat_id) = front_matter.category_id {
                // First priority: Use category_id from front matter
                categories.iter().find(|c| c.id == cat_id).cloned()
            } else if let Some(cat_id) = jd_category_id {
                // Second priority: Use category_id from path/filename
                categories.iter().find(|c| c.id == cat_id).cloned()
            } else if let Some(area_id) = front_matter.area_id.or(jd_area_id) {
                // Third priority: Find first category in the extracted area
                categories.iter()
                    .filter(|c| c.area_id == area_id)
                    .min_by_key(|c| c.id)
                    .cloned()
            } else {
                None
            };

            // Log what's happening for better debugging
            // leptos::logging::log!(
            //     "Processing file: {}, Category: {:?}, JD Identifier: {}",
            //     path.display(),
            //     category.as_ref().map(|c| &c.name),
            //     jd_identifier
            // );

            // Create project title
            let display_title = front_matter.title.clone();

            // Extract summary from front matter or first paragraph
            let summary = front_matter.summary
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| {
                    let (_, first_para) = extract_title_and_summary(&content);
                    first_para
                });

            Project {
                id: None, // Will be assigned later
                title: display_title,
                slug,
                summary,
                content,
                tech_stack: front_matter.tags,
                repo_url: None,
                live_url: None,
                thumbnail: None,
                created_at,
                updated_at: SystemTime::now(),
                jd_category: category,
                original_path: path.to_string_lossy().to_string(),
                jd_identifier,
                related_articles: front_matter.related_articles
            }
        })
        .collect()
}

// Helper to generate slug from title
fn slug_from_title(title: &str) -> String {
    let mut slug = String::new();

    for c in title.chars() {
        if c.is_alphanumeric() {
            slug.push(c.to_lowercase().next().unwrap());
        } else if c == ' ' {
            slug.push('-');
        }
    }

    slug
}

fn parse_date(date_str: &str) -> Option<SystemTime> {
    // Use the new parsing function from format.rs
    crate::utils::format::parse_date_string(date_str)
}
