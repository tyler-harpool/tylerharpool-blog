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
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug)]
pub struct ProjectDb {
    pub id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    // Note: tech_stack is NOT here because it's in a separate table
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub thumbnail: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub jd_category_id: Option<i64>,
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
    pub jd_category_id: Option<u8>,
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
    pub jd_category_id: Option<u8>,
}

// Johnny Decimal System structures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JDArea {
    pub id: u8,          // 10-19, 20-29, etc.
    pub name: String,    // e.g., "Technology"
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JDCategory {
    pub id: u8,          // 11, 12, 13, etc.
    pub area_id: u8,     // Which area this belongs to
    pub name: String,    // e.g., "Programming Languages"
    pub description: String,
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
        },
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
        JDCategory {
            id: 14,
            area_id: 10,
            name: "WebAssembly".into(),
            description: "WASM technologies, tools, and applications".into(),
        },

        // Digital Infrastructure (20-29)
        JDCategory {
            id: 21,
            area_id: 20,
            name: "Cloud Platforms".into(),
            description: "AWS, Azure, GCP, and cloud services".into(),
        },
        JDCategory {
            id: 22,
            area_id: 20,
            name: "DevOps & CI/CD".into(),
            description: "Continuous integration, deployment, and DevOps practices".into(),
        },
        JDCategory {
            id: 23,
            area_id: 20,
            name: "Containerization".into(),
            description: "Docker, Kubernetes, and container orchestration".into(),
        },

        // Government & Policy (30-39)
        JDCategory {
            id: 31,
            area_id: 30,
            name: "GovTech Initiatives".into(),
            description: "Government technology programs and digital transformation".into(),
        },
        JDCategory {
            id: 32,
            area_id: 30,
            name: "Digital Policy".into(),
            description: "Technology regulation, standards, and policy analysis".into(),
        },
        JDCategory {
            id: 33,
            area_id: 30,
            name: "Open Government".into(),
            description: "Open data, transparency, and civic tech".into(),
        },

        // Data & Analytics (40-49)
        JDCategory {
            id: 41,
            area_id: 40,
            name: "Data Science".into(),
            description: "Data analysis, visualization, and science techniques".into(),
        },
        JDCategory {
            id: 42,
            area_id: 40,
            name: "Machine Learning".into(),
            description: "AI, ML models, and intelligent systems".into(),
        },
        JDCategory {
            id: 43,
            area_id: 40,
            name: "Big Data".into(),
            description: "Large-scale data processing and analytics".into(),
        },

        // Industry Insights (50-59)
        JDCategory {
            id: 51,
            area_id: 50,
            name: "Case Studies".into(),
            description: "Real-world examples and implementation stories".into(),
        },
        JDCategory {
            id: 52,
            area_id: 50,
            name: "Tech Trends".into(),
            description: "Emerging technologies and industry directions".into(),
        },
        JDCategory {
            id: 53,
            area_id: 50,
            name: "Career Development".into(),
            description: "Professional growth, skills, and tech career advice".into(),
        },
    ]
}

// Helper function to find a category by ID
pub fn find_category_by_id(id: u8) -> Option<JDCategory> {
    get_all_categories().into_iter().find(|c| c.id == id)
}
