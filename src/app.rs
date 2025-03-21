use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, path,
};
use std::time::SystemTime;

// Import our page components
use crate::pages::{
    HomePage, AboutPage, ProjectPage, AreasPage, AreaDetailPage, CategoryDetailPage
};

// Import our regular components
use crate::components::Header;

// Import models
use crate::model::{
    Project, get_all_areas, get_all_categories, find_category_by_id
};

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
fn create_mock_projects() -> Vec<Project> {
    vec![
      Project {
          id: Some(1),
          title: "Building a Modern Web App with Leptos and Rust".into(),
          slug: "building-with-leptos".into(),
          summary: "An exploration of Rust's web framework ecosystem and how Leptos is pushing the boundaries.".into(),
          content: "# Project Details\n\nThis project was built using Rust and Leptos...".into(),
          tech_stack: vec!["Rust".into(), "Leptos".into(), "WebAssembly".into()],
          repo_url: Some("https://github.com/tylerharpool/building-with-leptos".into()),
          live_url: Some("https://building-with-leptos.tylerharpool.com".into()),
          thumbnail: None,
          created_at: SystemTime::now(),
          updated_at: SystemTime::now(),
          jd_category: find_category_by_id(12), // Web Frameworks
      },
      Project {
          id: Some(2),
          title: "Implementing Islands Architecture in a Rust Web Framework".into(),
          slug: "islands-architecture".into(),
          summary: "How partial hydration can improve performance while maintaining interactivity.".into(),
          content: "# Project Details\n\nThis project highlights how Islands Architecture...".into(),
          tech_stack: vec!["Rust".into(), "Leptos".into(), "Islands".into()],
          repo_url: Some("https://github.com/tylerharpool/islands-architecture".into()),
          live_url: Some("https://islands-architecture.tylerharpool.com".into()),
          thumbnail: None,
          // ~1 week ago
          created_at: SystemTime::now()
              .checked_sub(std::time::Duration::from_secs(7 * 24 * 3600))
              .unwrap_or(SystemTime::now()),
          updated_at: SystemTime::now(),
          jd_category: find_category_by_id(13), // Software Architecture
      },
      Project {
          id: Some(3),
          title: "Server Functions: Bridging the Frontend-Backend Divide".into(),
          slug: "server-functions".into(),
          summary: "Using Rust on both ends of the stack to create a seamless development experience.".into(),
          content: "# Project Details\n\nThis project demonstrates how server functions can unify front and back end...".into(),
          tech_stack: vec!["Rust".into(), "Leptos".into(), "Axum".into()],
          repo_url: Some("https://github.com/tylerharpool/server-functions".into()),
          live_url: Some("https://server-functions.tylerharpool.com".into()),
          thumbnail: None,
          // ~2 weeks ago
          created_at: SystemTime::now()
              .checked_sub(std::time::Duration::from_secs(14 * 24 * 3600))
              .unwrap_or(SystemTime::now()),
          updated_at: SystemTime::now(),
          jd_category: find_category_by_id(12), // Web Frameworks
      },
      // Adding a few more projects with different categories
      Project {
          id: Some(4),
          title: "Cloud-Native Deployment Strategies".into(),
          slug: "cloud-native-deployment".into(),
          summary: "Best practices for deploying applications in cloud environments with minimal downtime.".into(),
          content: "# Cloud-Native Deployment\n\nThis article explores various strategies...".into(),
          tech_stack: vec!["AWS".into(), "Docker".into(), "Kubernetes".into()],
          repo_url: None,
          live_url: None,
          thumbnail: None,
          created_at: SystemTime::now()
              .checked_sub(std::time::Duration::from_secs(3 * 24 * 3600))
              .unwrap_or(SystemTime::now()),
          updated_at: SystemTime::now(),
          jd_category: find_category_by_id(23), // Containerization
      },
      Project {
          id: Some(5),
          title: "Government Digital Transformation Initiatives".into(),
          slug: "gov-digital-transformation".into(),
          summary: "How governments are leveraging technology to improve service delivery and citizen engagement.".into(),
          content: "# Government Digital Transformation\n\nThis article examines recent initiatives...".into(),
          tech_stack: vec!["GovTech".into(), "Digital Services".into()],
          repo_url: None,
          live_url: None,
          thumbnail: None,
          created_at: SystemTime::now()
              .checked_sub(std::time::Duration::from_secs(10 * 24 * 3600))
              .unwrap_or(SystemTime::now()),
          updated_at: SystemTime::now(),
          jd_category: find_category_by_id(31), // GovTech Initiatives
      },

    ]
}
// -----------------------------------------
// Main App that sets up shared context + routes
// -----------------------------------------
#[component]
pub fn App() -> impl IntoView {
    // Provide context for metadata
    provide_meta_context();


    let (areas_signal, _) = signal(get_all_areas());
    let (categories_signal, _) = signal(get_all_categories());

    provide_context(areas_signal);
    provide_context(categories_signal);

    let projects = create_mock_projects();

    let (projects_signal, _) = signal(projects);
    provide_context(projects_signal);

    view! {
        <Stylesheet id="leptos" href="/pkg/tylerharpool-blog.css"/>
        <Title text="Tyler Harpool - Technology & Government Blog"/>
        <Header/>
        // Router for our routes
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route
                        path=StaticSegment("")
                        view=move || {
                            let projects = projects_signal.get();
                            view! { <HomePage projects=projects /> }
                        }
                    />
                    <Route path=StaticSegment("/about") view=AboutPage/>
                    // Project detail route
                    <Route path=path!("/projects/:slug") view=ProjectPage/>
                    // Routes for Johnny Decimal browsing
                    <Route path=path!("/areas") view=AreasPage/>
                    <Route path=path!("/areas/:id") view=AreaDetailPage/>
                    <Route path=path!("/categories/:id") view=CategoryDetailPage/>
                </Routes>
            </main>
        </Router>
    }
}
