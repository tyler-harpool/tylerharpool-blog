use leptos::prelude::*;
use crate::model::Project;
use crate::components::ProjectSearch;
use leptos_meta::Title;
use leptos::logging::log;

#[component]
pub fn HomePage(
    projects: Resource<Result<Vec<Project>, ServerFnError>>
) -> impl IntoView {
    let debug_data = move || {
        match projects.get() {
            Some(Ok(list)) => format!("Loaded {} projects", list.len()),
            Some(Err(e)) => format!("Error: {}", e),
            None => "Still loading...".to_string(),
        }
    };

    let refetch = move |_| {
        log!("Manually refetching projects");
        projects.refetch();
    };

    view! {
        <div class="container">
            // Debug panel (can be removed in production)
            <div style="background: #f0f0f0; padding: 10px; margin: 10px 0; border-radius: 5px;">
                <p>"Debug info: "{debug_data}</p>
                <button on:click=refetch>"Refetch Projects"</button>
            </div>

            <Title text="Tyler Harpool - Technology & Government Blog"/>
            <h1 class="site-title">"Tyler Harpool's Blog"</h1>
            <p class="intro-text">
                "Welcome to my blog where I share my thoughts and experiences on software architecture,
                enterprise solutions, and emerging technologies like Rust and WebAssembly."
            </p>

            // Make sure we're using the original CSS classes
            <div class="jd-system-section">
                <div class="recent-posts-section">
                    <h2 class="section-title">"Recent Posts"</h2>
                    <Suspense
                        fallback=move || view! {
                            <div class="loading-container">
                                <p class="loading-text">"Loading projects..."</p>
                            </div>
                        }
                    >
                        <Show
                            when=move || projects.get().is_some()
                            fallback=move || view! {
                                <div class="loading-container">
                                    <p class="loading-text">"Loading projects..."</p>
                                </div>
                            }
                        >
                            <Show
                                when=move || projects.get().unwrap().is_ok()
                                fallback=move || {
                                    let _err = projects.get().unwrap().unwrap_err();
                                    view! {
                                        <div class="error-container">
                                            <p class="error-text">"Error loading projects: {err.to_string()}"</p>
                                        </div>
                                    }
                                }
                            >
                                // This ensures the original container for ProjectSearch
                                <div class="projects-container">
                                    {move || {
                                        let project_list = projects.get().unwrap().unwrap();
                                        view! { <ProjectSearch projects=project_list /> }
                                    }}
                                </div>
                            </Show>
                        </Show>
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
