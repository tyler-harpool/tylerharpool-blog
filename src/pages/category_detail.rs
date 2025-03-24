use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::model::{ Project, find_category_by_id};

use crate::components::ProjectSearch;

#[component]
pub fn CategoryDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id_str = params.with(|params| params.get("id").unwrap_or_default().to_string());

    // Parse the ID from the URL path parameter
    let id = id_str.parse::<i64>().unwrap_or(0);

    // Get the category by ID
    let category = Memo::new(move |_| find_category_by_id(id));

    // Get the projects context
    let projects = use_context::<Resource<Result<Vec<Project>, ServerFnError>>>()
        .expect("Projects context not found!");

    // Create a signal to store the filtered projects
    let (filtered_projects, set_filtered_projects) = signal(Vec::new());

    // Effect to update filtered projects when resource changes
    Effect::new(move |_| {
        if let Some(Ok(all_projects)) = projects.get() {
            let filtered = all_projects
                .iter()
                .filter(|project| {
                    project.jd_category.as_ref()
                        .map(|cat| cat.id == id)
                        .unwrap_or(false)
                })
                .cloned()
                .collect::<Vec<Project>>();
            set_filtered_projects(filtered);
        }
    });

    view! {
        <div class="category-detail">
            <Show
                when=move || category.get().is_some()
                fallback=move || {
                    view! {
                        <h1>"Category not found"</h1>
                        <p>"The category you're looking for doesn't exist."</p>
                    }
                }
            >
                {move || {
                    // This is only called when category is Some
                    let cat = category.get().unwrap();
                    let cat_name = cat.name.clone();
                    let cat_description = cat.description.clone();

                    view! {
                        <h1>"Category: "{cat_name}</h1>
                        <p class="description">{cat_description}</p>

                        <h2>"Projects in this category"</h2>
                        <Suspense fallback=move || view! { <p>"Loading projects..."</p> }>
                            <Show
                                when=move || !filtered_projects.get().is_empty()
                                fallback=move || {
                                    view! { <p>"No projects in this category yet."</p> }
                                }
                            >
                                <div class="category-projects">
                                    // Create a new ProjectSearch with a new instance of the projects
                                    {move || {
                                        let projects_to_show = filtered_projects.get().clone();
                                        view! { <ProjectSearch projects=projects_to_show /> }
                                    }}
                                </div>
                            </Show>
                        </Suspense>
                    }
                }}
            </Show>
        </div>
    }
}
