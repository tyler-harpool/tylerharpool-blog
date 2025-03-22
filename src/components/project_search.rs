use leptos::prelude::*;
use crate::model::Project;

#[island]
pub fn ProjectSearch(
    #[prop(into)]
    projects: Vec<Project>
) -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());

    let filtered_projects = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            return projects.clone();
        }

        projects.iter()
            .filter(|project| {
                project.title.to_lowercase().contains(&query) ||
                project.summary.to_lowercase().contains(&query) ||
                project.tech_stack.iter().any(|tech| tech.to_lowercase().contains(&query))
            })
            .cloned()
            .collect::<Vec<Project>>()
    });

    view! {
        <div class="project-search">
            <div class="search-bar">
                <input
                    type="text"
                    placeholder="Search projects..."
                    on:input=move |ev| {
                        set_search_query(event_target_value(&ev));
                    }
                    prop:value=search_query
                />
            </div>

            <div class="projects-list">
                <Show
                    when=move || {
                        let filtered = filtered_projects.get();
                        filtered.is_empty() && !search_query.get().is_empty()
                    }
                    fallback=move || {
                        let filtered = filtered_projects.get();
                        view! {
                            <div class="project-grid">
                                <For
                                    each=move || filtered.clone()
                                    key=|project| project.id.unwrap_or(0)
                                    children=move |project| {
                                        // Clone everything we need
                                        let title = project.title.clone();
                                        let summary = project.summary.clone();
                                        let slug = project.slug.clone();
                                        let tech_stack = project.tech_stack.clone();
                                        let repo_url = project.repo_url.clone();
                                        let live_url = project.live_url.clone();

                                        view! {
                                            <div class="project-card">
                                                // Use the String directly, not as_str()
                                                <h3>{title}</h3>
                                                <p class="summary">{summary}</p>
                                                <div class="tech-stack">
                                                    <For
                                                        each=move || tech_stack.clone()
                                                        key=|tech| tech.clone()
                                                        children=move |tech| {
                                                            // Clone tech here too
                                                            let tech = tech.clone();
                                                            view! { <span class="tech-tag">{tech}</span> }
                                                        }
                                                    />
                                                </div>
                                                <div class="project-links">
                                                    <a href={format!("/projects/{}", slug)} class="view-details">
                                                        "View Details"
                                                    </a>
                                                    {repo_url.map(|url| {
                                                        view! {
                                                            <a href={url} class="repo-link" target="_blank">
                                                                "GitHub Repo"
                                                            </a>
                                                        }
                                                    })}
                                                    {live_url.map(|url| {
                                                        view! {
                                                            <a href={url} class="live-link" target="_blank">
                                                                "Live Demo"
                                                            </a>
                                                        }
                                                    })}
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                            </div>
                        }
                    }
                >
                    <p class="no-results">"No projects match your search."</p>
                </Show>
            </div>
        </div>
    }
}
