use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectListItem {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub tech_stack: Vec<String>,
    pub thumbnail: Option<String>,
    pub created_at: SystemTime
}
