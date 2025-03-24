use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, path,
};

use leptos::logging::log;
use leptos::task::spawn_local;
// Import our page components
use crate::pages::{
    HomePage, AboutPage, ProjectPage, AreasPage, AreaDetailPage, CategoryDetailPage
};

// Import our regular components
use crate::components::Header;

// Import models

use crate::model::{JDArea, JDCategory, Project};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProjectUpdate {
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


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NewProject {
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
#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::server_fn::ServerFnError;
    use sqlx::{Connection, SqliteConnection};  // Added Acquire trait

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:db.sqlite").await?)
    }
}
#[server]
pub async fn test_db_connection() -> Result<String, ServerFnError> {
    use crate::app::ssr::db;

    #[cfg(feature = "ssr")]
    log!("Testing database connection");

    match db().await {
        Ok(_) => {
            #[cfg(feature = "ssr")]
            log!("Database connection successful");
            Ok("Database connection successful".into())
        },
        Err(e) => {
            #[cfg(feature = "ssr")]
            log!("Database connection failed: {:?}", e);
            Err(e)
        }
    }
}
#[server]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    use crate::app::ssr::*;
    use chrono::{DateTime, Utc};
    use sqlx::Row;

    #[cfg(feature = "ssr")]
    eprintln!("=== get_projects server function called ===");

    let mut conn = match db().await {
        Ok(c) => {
            #[cfg(feature = "ssr")]
            eprintln!("Database connection successful");
            c
        },
        Err(e) => {
            #[cfg(feature = "ssr")]
            eprintln!("Database connection error: {:?}", e);
            return Err(e);
        }
    };

    // Use a raw query for more flexibility
    let rows = match sqlx::query(r#"
        SELECT
            p.id, p.title, p.slug, p.summary, p.content,
            p.repo_url, p.live_url, p.thumbnail,
            p.created_at, p.updated_at, p.jd_category_id,
            c.id as category_id, c.name as category_name,
            c.description as category_description, c.area_id,
            a.id as area_id, a.name as area_name, a.description as area_description
        FROM projects p
        LEFT JOIN jd_categories c ON p.jd_category_id = c.id
        LEFT JOIN jd_areas a ON c.area_id = a.id
        ORDER BY p.created_at DESC
    "#)
    .fetch_all(&mut conn)
    .await {
        Ok(rows) => rows,
        Err(e) => {
            #[cfg(feature = "ssr")]
            eprintln!("SQL error fetching projects: {:?}", e);
            return Err(ServerFnError::ServerError(format!("Database error: {}", e)));
        }
    };

    // Process rows into Project objects
    let mut projects = Vec::new();
    for row in rows {
        let id: Option<i64> = row.try_get("id").ok();

        // Get tech stack
        let tech_stack = if let Some(project_id) = id {
            match sqlx::query!(
                "SELECT technology FROM project_technologies WHERE project_id = ?",
                project_id
            )
            .fetch_all(&mut conn)
            .await {
                Ok(techs) => techs.into_iter().map(|r| r.technology).collect(),
                Err(_) => Vec::new(),
            }
        } else {
            Vec::new()
        };

        // Create JDCategory with area information
        let category_id: Option<i64> = row.try_get("category_id").ok();
        let area_id: Option<i64> = row.try_get("area_id").ok();

        let jd_category = if let (Some(cat_id), Some(a_id)) = (category_id, area_id) {
            // Create JDArea
            let area = JDArea {
                id: a_id,
                name: row.try_get("area_name").unwrap_or_default(),
                description: row.try_get("area_description").unwrap_or_default(),
            };

            // Create JDCategory with the area
            Some(JDCategory {
                id: cat_id,
                name: row.try_get("category_name").unwrap_or_default(),
                description: row.try_get("category_description").unwrap_or_default(),
                area_id: a_id,
                area: Some(area),
            })
        } else {
            None
        };

        // Parse dates
        let created_at_str: String = row.try_get("created_at").unwrap_or_default();
        let updated_at_str: String = row.try_get("updated_at").unwrap_or_default();

        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        // Build the full project
        projects.push(Project {
            id,
            title: row.try_get("title").unwrap_or_default(),
            slug: row.try_get("slug").unwrap_or_default(),
            summary: row.try_get("summary").unwrap_or_default(),
            content: row.try_get("content").unwrap_or_default(),
            tech_stack,
            repo_url: row.try_get("repo_url").ok(),
            live_url: row.try_get("live_url").ok(),
            thumbnail: row.try_get("thumbnail").ok(),
            created_at,
            updated_at,
            jd_category,
        });
    }

    #[cfg(feature = "ssr")]
    eprintln!("Returning {} fully processed projects", projects.len());

    Ok(projects)
}

#[server]
pub async fn get_project_by_slug(slug: String) -> Result<Option<Project>, ServerFnError> {
    use crate::app::ssr::*;

    use chrono::{DateTime, Utc};
    use crate::model::ProjectDbBasic;  // Import the simpler struct

    let mut conn = db().await?;

    // Get the project using the basic struct
    let db_project = sqlx::query_as!(
        ProjectDbBasic,
        "SELECT * FROM projects WHERE slug = ?",
        slug
    )
    .fetch_optional(&mut conn)
    .await?;

    // If project found, get tech_stack and build full Project
    if let Some(db_project) = db_project {
        // Get tech stack
        let tech_stack = if let Some(id) = db_project.id {
            sqlx::query!(
                "SELECT technology FROM project_technologies WHERE project_id = ?",
                id
            )
            .fetch_all(&mut conn)
            .await?
            .into_iter()
            .map(|r| r.technology)
            .collect()
        } else {
            Vec::new()
        };

        // Get category info directly using the id
        let jd_category = db_project.jd_category_id
            .and_then(|id| {
                crate::model::find_category_by_id(id)
            });

        // Parse the date strings to DateTime<Utc>
        let created_at = DateTime::parse_from_rfc3339(&db_project.created_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let updated_at = DateTime::parse_from_rfc3339(&db_project.updated_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        // Build the full project
        Ok(Some(Project {
            id: db_project.id,
            title: db_project.title,
            slug: db_project.slug,
            summary: db_project.summary,
            content: db_project.content,
            tech_stack,
            repo_url: db_project.repo_url,
            live_url: db_project.live_url,
            thumbnail: db_project.thumbnail,
            created_at,
            updated_at,
            jd_category,
        }))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn create_project(project: NewProject) -> Result<i64, ServerFnError> {
    use crate::app::ssr::*;
    use chrono::Utc;
    let mut conn = db().await?;

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
  use crate::app::ssr::*;
  use chrono::Utc;
  let mut conn = db().await?;

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

    // Update fields one by one
    if let Some(title) = &update.title {  // Changed from project_update.title
        sqlx::query!(
            "UPDATE projects SET title = ? WHERE id = ?",
            title,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    if let Some(slug) = &update.slug {  // Changed from project_update.slug
        sqlx::query!(
            "UPDATE projects SET slug = ? WHERE id = ?",
            slug,
            id
        )
        .execute(&mut conn)
        .await?;
    }

    // Continue changing the rest...
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
        now,
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
    use crate::app::ssr::*;

    let mut conn = db().await?;

    // Delete the project
    let result = sqlx::query!("DELETE FROM projects WHERE id = ?", id)
        .execute(&mut conn)
        .await?;

    Ok(result.rows_affected() > 0)
}
// This is your main "shell" function
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="Tyler Harpool's personal blog and project showcase"/>
                <meta name="keywords" content="web development, software engineering, projects, blog"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options islands=true/>
                <MetaTags/>
                <link rel="stylesheet" id="leptos" href="/pkg/tylerharpool-blog.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
            </head>
            <body>
                // Our top-level App
                <App/>
            </body>
        </html>
    }
}
// fn create_mock_projects() -> Vec<Project> {
//     vec![
//         Project {
//             id: Some(1),
//             title: "Building a Modern Web App with Leptos and Rust".into(),
//             slug: "building-with-leptos".into(),
//             summary: "An exploration of Rust's web framework ecosystem and how Leptos is pushing the boundaries.".into(),
//             content: "# Project Details\n\nThis project was built using Rust and Leptos...".into(),
//             tech_stack: vec!["Rust".into(), "Leptos".into(), "WebAssembly".into()],
//             repo_url: Some("https://github.com/tylerharpool/building-with-leptos".into()),
//             live_url: Some("https://building-with-leptos.tylerharpool.com".into()),
//             thumbnail: None,
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//             jd_category: find_category_by_id(12), // Web Frameworks
//         },
//         Project {
//             id: Some(2),
//             title: "Implementing Islands Architecture in a Rust Web Framework".into(),
//             slug: "islands-architecture".into(),
//             summary: "How partial hydration can improve performance while maintaining interactivity.".into(),
//             content: "# Project Details\n\nThis project highlights how Islands Architecture...".into(),
//             tech_stack: vec!["Rust".into(), "Leptos".into(), "Islands".into()],
//             repo_url: Some("https://github.com/tylerharpool/islands-architecture".into()),
//             live_url: Some("https://islands-architecture.tylerharpool.com".into()),
//             thumbnail: None,
//             // ~1 week ago
//             created_at: Utc::now() - Duration::days(7),
//             updated_at: Utc::now(),
//             jd_category: find_category_by_id(13), // Software Architecture
//         },
//         Project {
//             id: Some(3),
//             title: "Server Functions: Bridging the Frontend-Backend Divide".into(),
//             slug: "server-functions".into(),
//             summary: "Using Rust on both ends of the stack to create a seamless development experience.".into(),
//             content: "# Project Details\n\nThis project demonstrates how server functions can unify front and back end...".into(),
//             tech_stack: vec!["Rust".into(), "Leptos".into(), "Axum".into()],
//             repo_url: Some("https://github.com/tylerharpool/server-functions".into()),
//             live_url: Some("https://server-functions.tylerharpool.com".into()),
//             thumbnail: None,
//             // ~2 weeks ago
//             created_at: Utc::now() - Duration::days(14),
//             updated_at: Utc::now(),
//             jd_category: find_category_by_id(12), // Web Frameworks
//         },
//         // Adding a few more projects with different categories
//         Project {
//             id: Some(4),
//             title: "Cloud-Native Deployment Strategies".into(),
//             slug: "cloud-native-deployment".into(),
//             summary: "Best practices for deploying applications in cloud environments with minimal downtime.".into(),
//             content: "# Cloud-Native Deployment\n\nThis article explores various strategies...".into(),
//             tech_stack: vec!["AWS".into(), "Docker".into(), "Kubernetes".into()],
//             repo_url: None,
//             live_url: None,
//             thumbnail: None,
//             created_at: Utc::now() - Duration::days(3),
//             updated_at: Utc::now(),
//             jd_category: find_category_by_id(23), // Containerization
//         },
//         Project {
//             id: Some(5),
//             title: "Government Digital Transformation Initiatives".into(),
//             slug: "gov-digital-transformation".into(),
//             summary: "How governments are leveraging technology to improve service delivery and citizen engagement.".into(),
//             content: "# Government Digital Transformation\n\nThis article examines recent initiatives...".into(),
//             tech_stack: vec!["GovTech".into(), "Digital Services".into()],
//             repo_url: None,
//             live_url: None,
//             thumbnail: None,
//             created_at: Utc::now() - Duration::days(10),
//             updated_at: Utc::now(),
//             jd_category: find_category_by_id(31), // GovTech Initiatives
//         },
//     ]
// }
// -----------------------------------------
// Main App that sets up shared context + routes
// -----------------------------------------
#[component]
pub fn App() -> impl IntoView {
    // Provide context for metadata
    provide_meta_context();

    let (areas_signal, _) = signal(crate::model::get_all_areas());
    let (categories_signal, _) = signal(crate::model::get_all_categories());

    provide_context(areas_signal);
    provide_context(categories_signal);

    // Create the projects resource
    let projects = Resource::new(
        || (),
        |_| get_projects()
    );

    let test_db = move |_| {
        spawn_local(async {
            match test_db_connection().await {
                Ok(msg) => log!("DB test: {}", msg),
                Err(e) => log!("DB test error: {:?}", e),
            }
        });
    };

    // Important: Provide the projects resource as context for all routes
    provide_context(projects);

    view! {
        <Stylesheet id="leptos" href="/pkg/tylerharpool-blog.css"/>
        <Title text="Tyler Harpool - Technology & Government Blog"/>
        <Header/>
    <button on:click=test_db>"Test DB Connection"</button>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route
                        path=StaticSegment("")
                        view=move || {
                            view! {
                                <Suspense fallback=move || view! { <p>"Loading projects..."</p> }>
                                    <HomePage projects=projects />
                                </Suspense>
                            }
                        }
                    />
                    <Route path=StaticSegment("/about") view=AboutPage/>
                    <Route path=path!("/projects/:slug") view=ProjectPage/>
                    <Route path=path!("/areas") view=AreasPage/>
                    <Route path=path!("/areas/:id") view=AreaDetailPage/>
                    <Route path=path!("/categories/:id") view=CategoryDetailPage/>
                </Routes>
            </main>
        </Router>
    }
}
