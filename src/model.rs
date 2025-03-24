use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


// Base Project struct with all fields
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub jd_category: Option<JDCategory>,
}

// Database-specific Project struct for direct mapping - we'll add FromRow when sqlx is added
#[cfg_attr(feature = "ssr", derive(Debug, sqlx::FromRow))]
pub struct ProjectDb {
    // Existing project fields
    pub id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub thumbnail: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub jd_category_id: Option<i64>,

    // Category fields
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub category_description: Option<String>,
    pub area_id: Option<i64>,

    // Area fields
    pub area_name: Option<String>,
    pub area_description: Option<String>,
}

// For creating new projects
#[derive(Debug, Deserialize)]
pub struct CreateProject {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub tech_stack: Vec<String>,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub thumbnail: Option<String>,
    pub jd_category_id: Option<i64>,
}

// For updating existing projects
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateProject {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub tech_stack: Option<Vec<String>>,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub thumbnail: Option<String>,
    pub jd_category_id: Option<i64>,
}

// Johnny Decimal System structures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JDArea {
    pub id: i64,    // 10-19, 20-29, etc.
    pub name: String,    // e.g., "Technology"
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JDCategory {
    pub id: i64,          // 11, 12, 13, etc.
    pub area_id: i64,     // Which area this belongs to
    pub name: String,    // e.g., "Programming Languages"
    pub description: String,
    pub area: Option<JDArea>,
}

#[cfg_attr(feature = "ssr", derive(Debug, sqlx::FromRow))]
pub struct ProjectDbBasic {
    pub id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub thumbnail: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub jd_category_id: Option<i64>,
}

// Function to get all areas
pub fn get_all_areas() -> Vec<JDArea> {
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
}

// Function to get all categories
pub fn get_all_categories() -> Vec<JDCategory> {
    vec![
        // Technology & Development (10-19)
        JDCategory {
            id: 11,
            area_id: 10,
            name: "Programming Languages".into(),
            description: "Rust, Go, TypeScript, and other language-specific topics".into(),
            area: find_area_by_id(10),
        },
        JDCategory {
            id: 12,
            area_id: 10,
            name: "Web Frameworks".into(),
            description: "Leptos, React, Vue, and other web frameworks".into(),
            area: find_area_by_id(10),
        },
        JDCategory {
            id: 13,
            area_id: 10,
            name: "Software Architecture".into(),
            description: "Design patterns, best practices, and architectural approaches".into(),
            area: find_area_by_id(10),
        },
        JDCategory {
            id: 14,
            area_id: 10,
            name: "WebAssembly".into(),
            description: "WASM technologies, tools, and applications".into(),
            area: find_area_by_id(10),
        },

        // Digital Infrastructure (20-29)
        JDCategory {
            id: 21,
            area_id: 20,
            name: "Cloud Platforms".into(),
            description: "AWS, Azure, GCP, and cloud services".into(),
            area: find_area_by_id(20),
        },
        JDCategory {
            id: 22,
            area_id: 20,
            name: "DevOps & CI/CD".into(),
            description: "Continuous integration, deployment, and DevOps practices".into(),
            area: find_area_by_id(20),
        },
        JDCategory {
            id: 23,
            area_id: 20,
            name: "Containerization".into(),
            description: "Docker, Kubernetes, and container orchestration".into(),
            area: find_area_by_id(20),
        },

        // Government & Policy (30-39)
        JDCategory {
            id: 31,
            area_id: 30,
            name: "GovTech Initiatives".into(),
            description: "Government technology programs and digital transformation".into(),
            area: find_area_by_id(30),
        },
        JDCategory {
            id: 32,
            area_id: 30,
            name: "Digital Policy".into(),
            description: "Technology regulation, standards, and policy analysis".into(),
            area: find_area_by_id(30),
        },
        JDCategory {
            id: 33,
            area_id: 30,
            name: "Open Government".into(),
            description: "Open data, transparency, and civic tech".into(),
            area: find_area_by_id(30),
        },

        // Data & Analytics (40-49)
        JDCategory {
            id: 41,
            area_id: 40,
            name: "Data Science".into(),
            description: "Data analysis, visualization, and science techniques".into(),
            area: find_area_by_id(40),
        },
        JDCategory {
            id: 42,
            area_id: 40,
            name: "Machine Learning".into(),
            description: "AI, ML models, and intelligent systems".into(),
            area: find_area_by_id(40),
        },
        JDCategory {
            id: 43,
            area_id: 40,
            name: "Big Data".into(),
            description: "Large-scale data processing and analytics".into(),
            area: find_area_by_id(40),
        },

        // Industry Insights (50-59)
        JDCategory {
            id: 51,
            area_id: 50,
            name: "Case Studies".into(),
            description: "Real-world examples and implementation stories".into(),
            area: find_area_by_id(50),
        },
        JDCategory {
            id: 52,
            area_id: 50,
            name: "Tech Trends".into(),
            description: "Emerging technologies and industry directions".into(),
            area: find_area_by_id(50),
        },
        JDCategory {
            id: 53,
            area_id: 50,
            name: "Career Development".into(),
            description: "Professional growth, skills, and tech career advice".into(),
            area: find_area_by_id(50),
        },
    ]
}
// Helper function to find a category by ID
pub fn find_area_by_id(id: i64) -> Option<JDArea> {
    get_all_areas().into_iter().find(|a| a.id == id)
}

pub fn find_category_by_id(id: i64) -> Option<JDCategory> {
    get_all_categories().into_iter().find(|c| c.id == id)
}
