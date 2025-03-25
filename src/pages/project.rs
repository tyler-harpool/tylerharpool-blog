use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_meta::Title;
use crate::app::get_project_by_slug;
use crate::app::get_projects;
use crate::utils::format::format_date;
use crate::components::related_projects::RenderRelatedProjects;
use crate::components::{JohnnyDecimal, JohnnyDecimalBreadcrumbs};

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

                            // Convert to signals for reactivity instead of direct cloning
                            let jd_category = Memo::new(move |_| proj.jd_category.clone());
                            let has_jd_info = move || jd_category.get().is_some();

                            // Variable preparation
                            let title_text = format!("{} | Tyler Harpool", proj.title);
                            let project_title = proj.title.clone();
                            let project_summary = proj.summary.clone();
                            let project_content = proj.content.clone();
                            let formatted_date = format_date(proj.created_at);
                            let project_id = proj.id;

                            // Tech stack tags
                            let tech_stack = proj.tech_stack.iter().map(|tech| {
                                let tech_owned = tech.clone();
                                view! { <span class="tag">{tech_owned}</span> }
                            }).collect::<Vec<_>>();

                            // Related projects - using signals
                            let related_projects = if let Some(cat) = jd_category.get().as_ref() {
                                let category_id = cat.id;
                                let category_link = format!("/categories/{}", category_id);
                                let view_all_text = format!("View all projects in {}", cat.name.clone());

                                Some(view! {
                                    <RenderRelatedProjects
                                        project_id=project_id
                                        category_id=category_id
                                        category_link=category_link
                                        view_all_text=view_all_text
                                    />
                                })
                            } else {
                                None
                            };

                            view! {
                                <>
                                    <Title text=title_text />
                                    <div class="container">
                                        <article class="project-detail">
                                            <div class="project-jd-info">
                                                <Show
                                                    when=has_jd_info
                                                    fallback=move || {
                                                        view! {
                                                            <div class="jd-breadcrumbs empty">
                                                                <span>"No category information available"</span>
                                                            </div>
                                                        }
                                                    }
                                                >
                                                    // Both branches return the same type now
                                                    {move || {
                                                        if let Some(cat) = jd_category.get().as_ref() {
                                                            let area = cat.area.as_ref();
                                                            let area_id = cat.area_id.to_string();
                                                            let area_name = area.map_or("".to_string(), |a| a.name.clone());
                                                            let category_id = cat.id.to_string();
                                                            let category_name = cat.name.clone();

                                                            view! {
                                                                <JohnnyDecimalBreadcrumbs
                                                                    area_id=area_id
                                                                    area_name=area_name
                                                                    category_id=category_id
                                                                    category_name=category_name
                                                                />
                                                            }.into_any()
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }
                                                    }}
                                                </Show>

                                                <Show
                                                    when=has_jd_info
                                                    fallback=move || {
                                                        view! {
                                                            <div class="jd-container empty">
                                                                <div class="jd-notation">
                                                                    <span>"-"</span>
                                                                    <span class="jd-separator">.</span>
                                                                    <span>"-"</span>
                                                                </div>
                                                            </div>
                                                        }
                                                    }
                                                >
                                                    {move || {
                                                        if let Some(cat) = jd_category.get().as_ref() {
                                                            let area_id = cat.area_id.to_string();
                                                            let category_id = cat.id.to_string();
                                                            let category_name = cat.name.clone();

                                                            view! {
                                                                <JohnnyDecimal
                                                                    area_id=area_id
                                                                    category_id=category_id
                                                                    category_name=category_name
                                                                    show_name=true
                                                                />
                                                            }.into_any()
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }
                                                    }}
                                                </Show>
                                            </div>

                                            <header class="project-header">
                                                <h1 class="project-title">{project_title}</h1>
                                                <div class="project-meta">
                                                    <time class="project-date">{formatted_date}</time>
                                                </div>
                                            </header>

                                            <div class="project-summary">
                                                <p>{project_summary}</p>
                                            </div>

                                            <div class="tech-stack">
                                                <h3>"Technologies"</h3>
                                                <div class="tags">
                                                    {tech_stack}
                                                </div>
                                            </div>

                                            <div class="project-links">
                                                {proj.repo_url.as_ref().map(|url| {
                                                    let url_clone = url.clone();
                                                    view! {
                                                        <a href=url_clone class="btn btn-primary" target="_blank">"View Repository"</a>
                                                    }
                                                })}

                                                {proj.live_url.as_ref().map(|url| {
                                                    let url_clone = url.clone();
                                                    view! {
                                                        <a href=url_clone class="btn btn-secondary" target="_blank">"View Live Project"</a>
                                                    }
                                                })}
                                            </div>

                                            <div class="project-content markdown">
                                                <div inner_html={markdown_to_html(&project_content)}></div>
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
        // A very simple markdown to HTML converter
        // This isn't a full markdown parser, but should handle basic needs
        let mut html = String::new();
        let lines = markdown.lines();

        let mut in_list = false;
        let mut in_code_block = false;

        for line in lines {
            let trimmed = line.trim();

            // Handle headers
            if trimmed.starts_with("# ") {
                html.push_str(&format!("<h1>{}</h1>\n", &trimmed[2..]));
            } else if trimmed.starts_with("## ") {
                html.push_str(&format!("<h2>{}</h2>\n", &trimmed[3..]));
            } else if trimmed.starts_with("### ") {
                html.push_str(&format!("<h3>{}</h3>\n", &trimmed[4..]));
            }
            // Handle lists
            else if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
                if !in_list {
                    html.push_str("<ul>\n");
                    in_list = true;
                }
                html.push_str(&format!("<li>{}</li>\n", &trimmed[2..]));
            }
            // Handle code blocks
            else if trimmed.starts_with("```") {
                if !in_code_block {
                    html.push_str("<pre><code>\n");
                    in_code_block = true;
                } else {
                    html.push_str("</code></pre>\n");
                    in_code_block = false;
                }
            }
            // Handle blank lines
            else if trimmed.is_empty() {
                if in_list {
                    html.push_str("</ul>\n");
                    in_list = false;
                } else if !in_code_block {
                    html.push_str("<br />\n");
                } else {
                    html.push_str("\n");
                }
            }
            // Regular paragraph text
            else {
                if in_code_block {
                    html.push_str(line);
                    html.push_str("\n");
                } else {
                    html.push_str(&format!("<p>{}</p>\n", line));
                }
            }
        }

        // Close any open tags
        if in_list {
            html.push_str("</ul>\n");
        }
        if in_code_block {
            html.push_str("</code></pre>\n");
        }

        html
    }

    #[cfg(not(feature = "ssr"))]
    {
        markdown.to_string()
    }
}
