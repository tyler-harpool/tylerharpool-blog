use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use crate::utils::directory_scanner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub tech_stack: Vec<String>,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub thumbnail: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub jd_category: Option<JDCategory>,


    // New fields
    pub original_path: String,  // Store the original file path
    pub jd_identifier: String,  // Store the Johnny Decimal ID (e.g., "21.01")
    pub related_articles: Vec<String>
}

// Johnny Decimal System structures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JDArea {
    pub id: u8,          // 10-19, 20-29, etc.
    pub name: String,    // e.g., "Technology"
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JDCategory {
    pub id: u8,          // 11, 12, 13, etc.
    pub area_id: u8,     // Which area this belongs to
    pub name: String,    // e.g., "Programming Languages"
    pub description: String,
}


// Function to get all areas
pub fn get_all_areas() -> Vec<JDArea> {
    let areas = directory_scanner::get_cached_areas();

    if areas.is_empty() {
        // Fall back to hardcoded values if scanning failed
        vec![
            JDArea {
                id: 10,
                name: "Technology & Development".into(),
                description: "Programming languages, frameworks, and technical concepts".into(),
            },
            JDArea {
                id: 20,
                name: "Digital Infrastructure".into(),
                description: "Cloud services, DevOps, and system architecture".into(),
            },
            // Keep the rest of your existing areas...
            JDArea {
                id: 30,
                name: "Government & Policy".into(),
                description: "Government technology initiatives and digital policy".into(),
            },
            JDArea {
                id: 40,
                name: "Data & Analytics".into(),
                description: "Data science, machine learning, and analytics".into(),
            },
            JDArea {
                id: 50,
                name: "Industry Insights".into(),
                description: "Trends, case studies, and industry analysis".into(),
            },
        ]
    } else {
        areas
    }
}

// Function to get all categories
pub fn get_all_categories() -> Vec<JDCategory> {
    let categories = directory_scanner::get_cached_categories();

    if categories.is_empty() {
        // Fall back to hardcoded values if scanning failed
        vec![
            // Technology & Development (10-19)
            JDCategory {
                id: 11,
                area_id: 10,
                name: "Programming Languages".into(),
                description: "Rust, Go, TypeScript, and other language-specific topics".into(),
            },
            // Keep the rest of your existing categories...
            // Just showing a few here for brevity
            JDCategory {
                id: 12,
                area_id: 10,
                name: "Web Frameworks".into(),
                description: "Leptos, React, Vue, and other web frameworks".into(),
            },
            JDCategory {
                id: 13,
                area_id: 10,
                name: "Software Architecture".into(),
                description: "Design patterns, best practices, and architectural approaches".into(),
            },
            // ...rest of your hardcoded categories
        ]
    } else {
        categories
    }
}

// Helper function to find a category by ID
pub fn find_category_by_id(id: u8) -> Option<JDCategory> {
    get_all_categories().into_iter().find(|c| c.id == id)
}
