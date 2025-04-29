use leptos::prelude::*;
use leptos::logging::log;
use crate::model::Project;
use crate::utils::format::format_date;

#[island]
pub fn ProjectSearch(projects: Vec<Project>) -> impl IntoView {
    let (projects_signal, _) = signal(projects);
    let (search_query, set_search_query) = signal(String::new());

    // Pagination state
    let (current_page, set_current_page) = signal(1);
    let items_per_page = 5; // Adjust as needed

    // Sort & filter projects
    let filtered_projects = move || {
        let mut projects = projects_signal.get().clone();

        // Sort by date (most recent first)
        projects.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let q = search_query.get().to_lowercase();

        // Filter if search query exists
        if !q.is_empty() {
            projects = projects
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
                .collect::<Vec<_>>();
        }

        projects
    };

    // Calculate pagination metadata
    let total_items = move || filtered_projects().len();
    let total_pages = move || {
        let total = total_items();
        if total == 0 { 1 } else { (total + items_per_page - 1) / items_per_page }
    };

    // Get current page items
    let current_page_items = move || {
        let items = filtered_projects();
        let page = current_page.get();
        let start = (page - 1) * items_per_page;

        items.into_iter()
            .skip(start)
            .take(items_per_page)
            .collect::<Vec<_>>()
    };

    // Handle page navigation
    let go_to_page = move |page: usize| {
        set_current_page(page);
    };

    let next_page = move || {
        let current = current_page.get();
        let max = total_pages();
        if current < max {
            set_current_page(current + 1);
        }
    };

    let prev_page = move || {
        let current = current_page.get();
        if current > 1 {
            set_current_page(current - 1);
        }
    };

    // Boolean conditions for Show components
    let has_results = move || !current_page_items().is_empty();
    let should_show_pagination = move || total_pages() > 0;

    view! {
        <div>
            <input
                type="text"
                placeholder="Search projects..."
                class="search-input"
                on:input=move |ev| {
                    log!("User typed: {}", event_target_value(&ev));
                    set_search_query(event_target_value(&ev));
                    set_current_page(1); // Reset to first page on new search
                }
            />

            <div class="search-results">
                <Show
                    when=has_results
                    fallback=move || view! {
                        <div class="empty-search">
                            <p>"No results found. Try a different search term."</p>
                        </div>
                    }
                >
                    <div class="results-list">
                        {move || {
                            current_page_items().into_iter().map(|project| {
                                let formatted_date = format_date(project.created_at);
                                // Use the pre-extracted JD identifier
                                let decimal_id = if !project.jd_identifier.is_empty() {
                                    project.jd_identifier.clone()
                                } else {
                                    project.jd_category.as_ref().map_or("".to_string(), |cat| {
                                        format!("{}", cat.id)
                                    })
                                };

                                view! {
                                    <div class="search-result-item">
                                        <div class="result-header">
                                            {(!decimal_id.is_empty()).then(|| view! {
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
                                            <p class="result-summary">
                                                {project.summary.clone()}
                                            </p>
                                            <div class="result-meta">
                                                <span class="result-date">{formatted_date}</span>

                                                {project.jd_category.as_ref().map(|cat| {
                                                    view! {
                                                        <div class="jd-info">
                                                            <a href={format!("/areas/{}", cat.area_id)} class="jd-area-badge">
                                                                {format!("{}-{}", cat.area_id, cat.area_id + 9)}
                                                            </a>
                                                            <a href={format!("/categories/{}", cat.id)} class="jd-category-badge">
                                                                {cat.id}
                                                            </a>
                                                        </div>
                                                    }
                                                })}

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
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </Show>

                // Pagination controls
                <Show when=should_show_pagination>
                    <div class="pagination-controls">
                        <div class="pagination-info">
                            {move || format!("Page {} of {}", current_page.get(), total_pages())}
                        </div>

                        <div class="pagination-buttons">
                            <button
                                class="pagination-btn prev-btn"
                                disabled={move || current_page.get() <= 1}
                                on:click=move |_| prev_page()
                            >
                                "Previous"
                            </button>

                            {move || {
                                let total = total_pages();
                                let current = current_page.get();

                                // Create page number buttons
                                let mut page_buttons = Vec::new();

                                // Show max 5 page numbers
                                let display_count = 5.min(total);

                                // Calculate the start page number
                                let half = display_count / 2;
                                let mut start_page = if current > half {
                                    if current + half > total {
                                        total - display_count + 1
                                    } else {
                                        current - half
                                    }
                                } else {
                                    1
                                };

                                start_page = start_page.max(1);

                                for page in start_page..=(start_page + display_count - 1).min(total) {
                                    let is_active = page == current;
                                    let page_for_closure = page;

                                    page_buttons.push(view! {
                                        <button
                                            class={if is_active { "pagination-btn page-btn active" } else { "pagination-btn page-btn" }}
                                            on:click=move |_| go_to_page(page_for_closure)
                                        >
                                            {page}
                                        </button>
                                    });
                                }

                                page_buttons
                            }}

                            <button
                                class="pagination-btn next-btn"
                                disabled={move || current_page.get() >= total_pages()}
                                on:click=move |_| next_page()
                            >
                                "Next"
                            </button>
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}
