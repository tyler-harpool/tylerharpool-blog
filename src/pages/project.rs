use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_meta::Title;
use crate::app::get_project_by_slug;
use crate::app::get_projects;
use crate::utils::format::format_date;
use crate::components::related_projects::RenderRelatedProjects;

#[component]
pub fn ProjectPage() -> impl IntoView {
    // Get the slug from the URL
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").map(|s| s.clone()).unwrap_or_default());

    // Create a resource for just this project
    let project = Resource::new(
        slug,
        |current_slug| async move {
            if !current_slug.is_empty() {
                get_project_by_slug(current_slug).await
            } else {
                Ok(None)
            }
        }
    );

    // Create a resource for all projects
    let all_projects = Resource::new(
        || (), |_| async move { get_projects().await }
    );

    view! {
        <div class="project-page">
            <Suspense fallback=move || view! { <p class="loading">"Loading project details..."</p> }>
                {move || {
                    match project.get() {
                        None => view! { <p class="loading">"Loading project details..."</p> }.into_any(),
                        Some(Ok(Some(proj))) => {
                            // Set up context for related projects
                            let (projects_signal, _) = signal(
                                all_projects.get().and_then(|r| r.ok()).unwrap_or_default()
                            );
                            provide_context(projects_signal);

                            // Basic project data
                            let title_text = format!("{} | Tyler Harpool", proj.title);
                            let formatted_date = format_date(proj.created_at);
                            let project_id = proj.id;

                            // Process JD category data if available
                            let (breadcrumbs, decimal_display, related_projects) =
                                if let Some(cat) = &proj.jd_category {
                                    // Create breadcrumbs
                                    let area = cat.area.as_ref();
                                    let area_id = cat.area_id;
                                    let area_name = area.map_or("".to_string(), |a| a.name.clone());

                                    let breadcrumbs = view! {
                                        <div class="project-breadcrumbs">
                                            <a href="/areas"><span class="breadcrumb-label">"Areas"</span></a>
                                            " / "
                                            <a href={format!("/areas/{}", area_id)}>
                                                <span class="breadcrumb-area-code">{area_id}</span>
                                                <span class="breadcrumb-label">{area_name}</span>
                                            </a>
                                            " / "
                                            <a href={format!("/categories/{}", cat.id)}>
                                                <span class="breadcrumb-category-code">{cat.id}</span>
                                                <span class="breadcrumb-label">{cat.name.clone()}</span>
                                            </a>
                                        </div>
                                    };

                                    // Create decimal display
                                    let decimal = format!("{}.{}", cat.area_id, cat.id);
                                    let decimal_display = view! {
                                        <div class="project-decimal-container">
                                            <div class="project-decimal">{decimal}</div>
                                            <div class="project-category-label">
                                                <div class="project-category-id">{cat.id}</div>
                                                <div class="project-category-name">{cat.name.clone()}</div>
                                            </div>
                                        </div>
                                    };

                                    // Create related projects component
                                    let category_id = cat.id;
                                    let category_link = format!("/categories/{}", category_id);
                                    let view_all_text = format!("View all projects in {}", cat.name.clone());

                                    let related = view! {
                                        <RenderRelatedProjects
                                            project_id=project_id
                                            category_id=category_id
                                            category_link=category_link
                                            view_all_text=view_all_text
                                        />
                                    };

                                    (Some(breadcrumbs), Some(decimal_display), Some(related))
                                } else {
                                    (None, None, None)
                                };

                            // Links and tech tags
                            let tech_stack = proj.tech_stack.iter().map(|tech| {
                                view! { <span class="tag">{tech.clone()}</span> }
                            }).collect::<Vec<_>>();

                            // Final view
                            view! {
                                <>
                                    <Title text=title_text />
                                    <div class="container">
                                        <article class="project-detail">
                                            <div class="project-jd-info">
                                                {breadcrumbs}
                                                {decimal_display}
                                            </div>

                                            <header class="project-header">
                                                <h1 class="project-title">{proj.title.clone()}</h1>
                                                <div class="project-meta">
                                                    <time class="project-date">{formatted_date}</time>
                                                </div>
                                            </header>

                                            <div class="project-summary">
                                                <p>{proj.summary.clone()}</p>
                                            </div>

                                            <div class="tech-stack">
                                                <h3>"Technologies"</h3>
                                                <div class="tags">{tech_stack}</div>
                                            </div>

                                            <div class="project-links">
                                                {proj.repo_url.as_ref().map(|url| view! {
                                                    <a href={url.clone()} class="btn btn-primary" target="_blank">
                                                        "View Repository"
                                                    </a>
                                                })}

                                                {proj.live_url.as_ref().map(|url| view! {
                                                    <a href={url.clone()} class="btn btn-secondary" target="_blank">
                                                        "View Live Project"
                                                    </a>
                                                })}
                                            </div>

                                            <div class="project-content markdown">
                                                <div inner_html=markdown_to_html(&proj.content)></div>
                                            </div>

                                            {related_projects}

                                            <div class="navigation-links">
                                                <a href="/" class="btn btn-back">"← Back to Projects"</a>
                                            </div>
                                        </article>
                                    </div>
                                </>
                            }.into_any()
                        },
                        Some(Ok(None)) => view! { <div class="error">"Project not found."</div> }.into_any(),
                        Some(Err(e)) => view! { <div class="error">"Error loading project: " {e.to_string()}</div> }.into_any(),
                    }
                }}
            </Suspense>
        </div>
    }
}

// Helper function to convert markdown to HTML
fn markdown_to_html(markdown: &str) -> String {
    #[cfg(feature = "ssr")]
    {
        use pulldown_cmark::{Parser, Options, html};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }

    #[cfg(not(feature = "ssr"))]
    {
        // On the client, we'll use the pre-rendered HTML
        markdown.to_string()
    }
}
