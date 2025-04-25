use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, path,
};
use leptos::logging::log;
use crate::utils::jd_content_loader::markdown_to_projects;

// Import our page components
use crate::pages::{
    HomePage, AboutPage, ProjectPage, AreasPage, AreaDetailPage, CategoryDetailPage
};

// Import our regular components
use crate::components::Header;

// Import models
use crate::model::{
    Project, get_all_areas, get_all_categories
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

// Replace the create_mock_projects function with:
fn load_projects() -> Vec<Project> {
    // Get areas and categories using our dynamic functions
    let areas = get_all_areas();
    let categories = get_all_categories();

    // Log what we found
    log!("Loaded {} areas and {} categories", areas.len(), categories.len());

    let mut projects = markdown_to_projects("content/blog", &areas, &categories);

    // Add IDs to the projects
    for (i, project) in projects.iter_mut().enumerate() {
        project.id = Some((i + 1) as i64);
    }

    log!("Loaded {} projects", projects.len());
    projects
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

    let projects = load_projects();

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
