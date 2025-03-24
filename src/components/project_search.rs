use leptos::prelude::*;
use crate::model::Project;
use crate::utils::format::format_date;
use leptos::logging::log;

// 1. Search Input Island Component
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
        <div>
            <SearchInput
                query=search_query
                set_query=set_search_query
            />

            <SearchResults
                projects=filtered_projects
                search_query=search_query.into()
            />
        </div>
    }
}

// 2. Search Input Component
#[component]
fn SearchInput(
    query: ReadSignal<String>,
    set_query: WriteSignal<String>
) -> impl IntoView {
    view! {
        <input
            type="text"
            placeholder="Search projects..."
            class="search-input"
            on:input=move |ev| {
                let value = event_target_value(&ev);
                log!("User typed: {}", value);
                set_query(value);
            }
            prop:value=query
        />
    }.into_any();
}

// 3. Search Results Component
#[component]
fn SearchResults(
    projects: Memo<Vec<Project>>,
    search_query: Signal<String>
) -> impl IntoView {
    view! {
        <Show
            when=move || {
                let filtered = projects.get();
                !filtered.is_empty() || search_query.get().is_empty()
            }
            fallback=move || {
                view! { <p class="no-results">"No projects match your search."</p> }
            }
        >
            <div class="search-results">
                <For
                    each=move || projects.get()
                    key=|project| project.id.unwrap_or(0)
                    children=move |project| {
                        view! { <ProjectCard project=project.clone() /> }
                    }
                />
            </div>
        </Show>
    }
}

// 4. Project Card Component
#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    // Create stored values for data that needs to be accessed in reactive contexts
    let formatted_date = StoredValue::new(format_date(project.created_at));

    // Calculate and store decimal ID
    let decimal_id = StoredValue::new({
        project.jd_category.as_ref().map_or("".to_string(), |cat| {
            // Since we're transitioning to a new data model, let's be defensive
            format!("{}.{}", cat.area_id, cat.id)
        })
    });

    let has_category = project.jd_category.is_some();
    let title = StoredValue::new(project.title);
    let summary = StoredValue::new(project.summary);
    let slug = StoredValue::new(project.slug);
    let tech_stack = StoredValue::new(project.tech_stack);

    view! {
        <div class="search-result-item">
            <div class="result-header">
                <Show
                    when=move || has_category
                    fallback=|| view! { <div></div> }
                >
                    <div class="result-decimal-container">
                        <span class="result-decimal">{move || decimal_id.get_value()}</span>
                    </div>
                </Show>

                <div class="result-title-container">
                    <a href={move || format!("/projects/{}", slug.get_value())} class="result-title">
                        {move || title.get_value()}
                    </a>
                </div>
            </div>

            <div class="result-content">
                <p class="result-summary">{move || summary.get_value()}</p>
                <div class="result-meta">
                    <span class="result-date">{move || formatted_date.get_value()}</span>
                    // Fixed tech tags rendering
                    <div class="result-tags">
                        {move || {
                            tech_stack.get_value().iter().map(|tech| {
                                // Convert &String to an owned String for the closure
                                let tech_str = tech.clone();
                                view! { <span class="result-tag">{tech_str}</span> }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
