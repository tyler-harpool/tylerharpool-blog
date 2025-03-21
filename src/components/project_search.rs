use leptos::prelude::*;
use leptos::logging::log;
use crate::model::Project;
use crate::utils::format::format_date;

#[island]
pub fn ProjectSearch(projects: Vec<Project>) -> impl IntoView {
    let (projects_signal, _) = signal(projects);
    let (search_query, set_search_query) = signal(String::new());

    // Filtered projects implementation
    let filtered_projects = move || {
        let mut projects = projects_signal.get().clone();
        projects.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let q = search_query.get().to_lowercase();

        if q.is_empty() {
            return projects;
        }

        projects
            .into_iter()
            .filter(|p| {
                p.title.to_lowercase().contains(&q) ||
                p.summary.to_lowercase().contains(&q) ||
                p.tech_stack.iter().any(|tech| tech.to_lowercase().contains(&q)) ||
                p.jd_category.as_ref().map_or(false, |c|
                    c.name.to_lowercase().contains(&q) ||
                    c.description.to_lowercase().contains(&q)
                )
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div>
            <input
                type="text"
                placeholder="Search projects..."
                class="search-input"
                on:input=move |ev| {
                    log!("User typed: {}", event_target_value(&ev));
                    set_search_query(event_target_value(&ev));
                }
            />

            <div class="search-results">
                {move || filtered_projects().into_iter().map(|project| {
                    let formatted_date = format_date(project.created_at);
                    let decimal_id = project.jd_category.as_ref().map_or("".to_string(), |cat| {
                        format!("{}.{}", cat.id, project.id.unwrap_or(0))
                    });

                    view! {
                        <div class="search-result-item">
                            <div class="result-header">
                                {project.jd_category.as_ref().map(|_| view! {
                                    <div class="result-decimal-container">
                                        <span class="result-decimal">{decimal_id}</span>
                                    </div>
                                })}

                                <div class="result-title-container">
                                    <a href={format!("/projects/{}", project.slug)} class="result-title">
                                        {project.title}
                                    </a>
                                </div>
                            </div>

                            <div class="result-content">
                                <p class="result-summary">{project.summary.clone()}</p>
                                <div class="result-meta">
                                    <span class="result-date">{formatted_date}</span>
                                    <div class="result-tags">
                                        {project.tech_stack.iter().map(|tech| {
                                            view! {
                                                <span class="result-tag">{tech.clone()}</span>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
