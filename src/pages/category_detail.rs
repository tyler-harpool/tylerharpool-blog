use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::app::get_projects;
use crate::model::{JDCategory, Project};
use crate::components::{CategoryBadge, JohnnyDecimal, ProjectSearch};

#[component]
pub fn CategoryDetailPage() -> impl IntoView {
    // Get the category ID from URL params
    let params = use_params_map();
    let id_str = Signal::derive(move || params.with(|p| p.get("id").unwrap_or_default().to_string()));
    let category_id = Signal::derive(move || id_str.get().parse::<i64>().unwrap_or(0));

    // Create a resource for all projects
    let all_projects = Resource::new(
        || (), |_| async move { get_projects().await }
    );

    // Create reactive signals for our state
    let (selected_category, set_selected_category) = create_signal(None::<JDCategory>);
    let (filtered_projects, set_filtered_projects) = create_signal(Vec::<Project>::new());

    // Update our reactive signals when the resource data changes
    create_effect(move |_| {
        if let Some(Ok(projects)) = all_projects.get() {
            // Find category from projects
            for project in &projects {
                if let Some(cat) = &project.jd_category {
                    if cat.id == category_id.get() {
                        set_selected_category.set(Some(cat.clone()));
                        break;
                    }
                }
            }

            // Filter projects by category
            let filtered = projects
                .iter()
                .filter(|p| p.jd_category.as_ref().map(|c| c.id == category_id.get()).unwrap_or(false))
                .cloned()
                .collect::<Vec<Project>>();

            set_filtered_projects.set(filtered);
        }
    });

    // Derive helper signals for the view
    let has_category = Signal::derive(move || selected_category.get().is_some());
    let has_projects = Signal::derive(move || !filtered_projects.get().is_empty());

    view! {
        <div class="category-detail container">
            <Show
                when=has_category
                fallback=move || {
                    view! {
                        <div class="category-not-found">
                            <h1>"Category not found"</h1>
                            <p>"The category you're looking for doesn't exist or hasn't been loaded yet."</p>
                        </div>
                    }
                }
            >
                <div class="category-content">
                    <Show
                        when=has_category
                        fallback=|| view! { <div></div> }
                    >
                        {move || {
                            // Clone the category data to avoid ownership issues in the closure
                            let cat_data = selected_category.get();
                            let Some(cat) = cat_data else { return view! { <div></div> }.into_any() };

                            // Clone all values we'll use in the template to avoid borrow checker issues
                            let area_id = cat.area_id.to_string();
                            let category_id_str = cat.id.to_string();
                            let category_name = cat.name.clone();
                            let category_description = cat.description.clone();

                            view! {
                                <div class="category-header">
                                    <div class="category-title-section">
                                        <CategoryBadge id=category_id_str.clone() />
                                        <h1>{category_name.clone()}</h1>
                                    </div>

                                    <div class="category-info">
                                        <JohnnyDecimal
                                            area_id=area_id
                                            category_id=category_id_str
                                            category_name=String::new()
                                            show_name=false
                                        />
                                        <p class="category-description">{category_description}</p>
                                    </div>
                                </div>

                                <h2>"Projects in this category"</h2>

                                <Suspense fallback=move || view! { <p class="loading">"Loading projects..."</p> }>
                                    <Show
                                        when=has_projects
                                        fallback=move || {
                                            view! {
                                                <div class="empty-state">
                                                    <p>"No projects in this category yet."</p>
                                                </div>
                                            }
                                        }
                                    >
                                        <div class="category-projects">
                                            {move || {
                                                // Clone the projects data to avoid ownership issues
                                                let projects_data = filtered_projects.get().clone();
                                                view! { <ProjectSearch projects=projects_data /> }
                                            }}
                                        </div>
                                    </Show>
                                </Suspense>
                            }.into_any()
                        }}
                    </Show>
                </div>
            </Show>
        </div>
    }
}
