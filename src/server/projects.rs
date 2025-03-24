#![cfg(feature = "ssr")]
use crate::model::{JdArea, JdCategory, Project, NewProject, ProjectUpdate};
use crate::server::db::get_connection;
use leptos::server_fn::ServerFnError;
use leptos::server;
use sqlx::Row;
use chrono::Utc;

#[server]
pub async fn get_all_projects() -> Result<Vec<Project>, ServerFnError> {
    let mut conn = get_connection().await?;

    // Query for all projects with joins to categories and areas
    let query = r#"
        SELECT
            p.id, p.title, p.slug, p.summary, p.content, p.repo_url, p.live_url,
            p.thumbnail, p.created_at, p.updated_at, p.jd_category_id,
            c.id as category_id, c.name as category_name, c.description as category_description, c.area_id,
            a.id as area_id, a.name as area_name, a.description as area_description
        FROM projects p
        LEFT JOIN jd_categories c ON p.jd_category_id = c.id
        LEFT JOIN jd_areas a ON c.area_id = a.id
        ORDER BY p.created_at DESC
    "#;

    let rows = sqlx::query(query)
        .fetch_all(&mut conn)
        .await?;

    let mut projects = Vec::new();

    for row in rows {
        let id: i64 = row.get("id");
        let jd_category_id: Option<i64> = row.get("jd_category_id");

        // Get category data if present
        let jd_category = if jd_category_id.is_some() {
            let category_id: Option<i64> = row.get("category_id");

            if let Some(category_id) = category_id {
                let area_id: i64 = row.get("area_id");

                Some(JdCategory {
                    id: category_id,
                    name: row.get("category_name"),
                    description: row.get("category_description"),
                    area_id,
                    area: Some(JdArea {
                        id: row.get("area_id"),
                        name: row.get("area_name"),
                        description: row.get("area_description"),
                    }),
                })
            } else {
                None
            }
        } else {
            None
        };

        // Get technologies for this project
        let tech_query = "SELECT technology FROM project_technologies WHERE project_id = ?";
        let technologies = sqlx::query(tech_query)
            .bind(id)
            .fetch_all(&mut conn)
            .await?
            .into_iter()
            .map(|row| row.get("technology"))
            .collect::<Vec<String>>();

        // Build the project
        projects.push(Project {
            id: Some(id),
            title: row.get("title"),
            slug: row.get("slug"),
            summary: row.get("summary"),
            content: row.get("content"),
            repo_url: row.get("repo_url"),
            live_url: row.get("live_url"),
            thumbnail: row.get("thumbnail"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            jd_category_id,
            jd_category,
            tech_stack: technologies,
        });
    }

    Ok(projects)
}

#[server]
pub async fn get_project_by_slug(slug: String) -> Result<Option<Project>, ServerFnError> {
    let mut conn = get_connection().await?;

    // Query for the project
    let query = r#"
        SELECT
            p.id, p.title, p.slug, p.summary, p.content, p.repo_url, p.live_url,
            p.thumbnail, p.created_at, p.updated_at, p.jd_category_id,
            c.id as category_id, c.name as category_name, c.description as category_description, c.area_id,
            a.id as area_id, a.name as area_name, a.description as area_description
        FROM projects p
        LEFT JOIN jd_categories c ON p.jd_category_id = c.id
        LEFT JOIN jd_areas a ON c.area_id = a.id
        WHERE p.slug = ?
        LIMIT 1
    "#;

    let result = sqlx::query(query)
        .bind(slug)
        .fetch_optional(&mut conn)
        .await?;

    if let Some(row) = result {
        let id: i64 = row.get("id");
        let jd_category_id: Option<i64> = row.get("jd_category_id");

        // Process creation/update dates as strings from DB
        let created_at: String = row.get("created_at");
        let updated_at: String = row.get("updated_at");

        // Get category data if present
        let jd_category = if let Some(_) = jd_category_id {
            let category_id: Option<i64> = row.get("category_id");

            if let Some(category_id) = category_id {
                let area_id: i64 = row.get("area_id");

                Some(JdCategory {
                    id: category_id,
                    name: row.get("category_name"),
                    description: row.get("category_description"),
                    area_id,
                    area: Some(JdArea {
                        id: row.get("area_id"),
                        name: row.get("area_name"),
                        description: row.get("area_description"),
                    }),
                })
            } else {
                None
            }
        } else {
            None
        };

        // Get technologies for this project
        let tech_query = "SELECT technology FROM project_technologies WHERE project_id = ?";
        let technologies = sqlx::query(tech_query)
            .bind(id)
            .fetch_all(&mut conn)
            .await?
            .into_iter()
            .map(|row| row.get("technology"))
            .collect::<Vec<String>>();

        // Build the project
        let project = Project {
            id: Some(id),
            title: row.get("title"),
            slug: row.get("slug"),
            summary: row.get("summary"),
            content: row.get("content"),
            repo_url: row.get("repo_url"),
            live_url: row.get("live_url"),
            thumbnail: row.get("thumbnail"),
            created_at,  // Already a string from the DB
            updated_at,  // Already a string from the DB
            jd_category_id,  // Include the category ID
            jd_category,  // Include the category object
            tech_stack: technologies,
        };

        Ok(Some(project))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn create_project(project: NewProject) -> Result<i64, ServerFnError> {
    let mut conn = get_connection().await?;

    // Current time for timestamps
    let now = Utc::now();
    let now_str = now.to_rfc3339(); // Convert to ISO8601/RFC3339 string

    // Convert category ID from u8 to i64 if present
    let category_id = project.jd_category_id.map(|id| id as i64);

    // Insert the project
    let id = sqlx::query!(
        r#"
        INSERT INTO projects
        (title, slug, summary, content, repo_url, live_url, thumbnail, created_at, updated_at, jd_category_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        project.title,
        project.slug,
        project.summary,
        project.content,
        project.repo_url,
        project.live_url,
        project.thumbnail,
        now_str,  // Use string format
        now_str,
        category_id  // Use converted i64
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    // Insert tech stack
    for tech in &project.tech_stack {
        sqlx::query!(
            "INSERT INTO project_technologies (project_id, technology) VALUES (?, ?)",
            id,
            tech
        )
        .execute(&mut conn)
        .await?;
    }

    Ok(id)
}

#[server]
pub async fn update_project(id: i64, update: ProjectUpdate) -> Result<bool, ServerFnError> {
    let mut conn = get_connection().await?;

    // Check if the project exists first - simpler approach
    let count = sqlx::query!(
        "SELECT COUNT(*) as count FROM projects WHERE id = ?",
        id
    )
    .fetch_one(&mut conn)
    .await?
    .count;

    if count == 0 {
        return Ok(false);
    }

    // Current time for timestamps
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    
    // Update fields one by one
    if let Some(title) = &update.title {
        sqlx::query!(
            "UPDATE projects SET title = ? WHERE id = ?",
            title,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    if let Some(slug) = &update.slug {
        sqlx::query!(
            "UPDATE projects SET slug = ? WHERE id = ?",
            slug,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    if let Some(summary) = &update.summary {
        sqlx::query!(
            "UPDATE projects SET summary = ? WHERE id = ?",
            summary,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    if let Some(content) = &update.content {
        sqlx::query!(
            "UPDATE projects SET content = ? WHERE id = ?",
            content,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    if update.repo_url.is_some() {
        sqlx::query!(
            "UPDATE projects SET repo_url = ? WHERE id = ?",
            update.repo_url,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    if update.live_url.is_some() {
        sqlx::query!(
            "UPDATE projects SET live_url = ? WHERE id = ?",
            update.live_url,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    if update.thumbnail.is_some() {
        sqlx::query!(
            "UPDATE projects SET thumbnail = ? WHERE id = ?",
            update.thumbnail,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    if update.jd_category_id.is_some() {
        sqlx::query!(
            "UPDATE projects SET jd_category_id = ? WHERE id = ?",
            update.jd_category_id,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    // Always update the updated_at timestamp
    sqlx::query!(
        "UPDATE projects SET updated_at = ? WHERE id = ?",
        now_str,
        id
    )
    .execute(&mut conn)
    .await?;

    // Handle tech stack updates
    if let Some(tech_stack) = &update.tech_stack {
        // Delete existing tech stack
        sqlx::query!("DELETE FROM project_technologies WHERE project_id = ?", id)
            .execute(&mut conn)
            .await?;

        // Insert new tech stack
        for tech in tech_stack {
            sqlx::query!(
                "INSERT INTO project_technologies (project_id, technology) VALUES (?, ?)",
                id,
                tech
            )
            .execute(&mut conn)
            .await?;
        }
    }

    Ok(true)
}

#[server]
pub async fn delete_project(id: i64) -> Result<bool, ServerFnError> {
    let mut conn = get_connection().await?;

    // Delete the project
    let result = sqlx::query!("DELETE FROM projects WHERE id = ?", id)
        .execute(&mut conn)
        .await?;

    Ok(result.rows_affected() > 0)
}