use leptos::prelude::*;
use crate::model::Project;
use crate::components::ProjectSearch;

#[component]
pub fn TaggedSearch(
    projects: Vec<Project>,
    #[prop(default = String::new())] active_tag: String
) -> impl IntoView {
    // Create a signal to store active_tag so we can use it multiple times
    let (tag_signal, _) = signal(active_tag);

    // Pre-filter projects by tag
    let filtered_projects = move || {
        let tag = tag_signal.get();

        if tag.is_empty() {
            // No tag filter, return all projects
            projects.clone()
        } else {
            // Filter projects by tag
            let tag_lower = tag.to_lowercase();
            projects.iter()
                .filter(|p| p.tech_stack.iter().any(|t| t.to_lowercase() == tag_lower))
                .cloned()
                .collect()
        }
    };

    view! {
        <div>
            // Tag filter banner
            {move || {
                let tag = tag_signal.get();
                (!tag.is_empty()).then(|| view! {
                    <div class="tag-filter-banner">
                        <p>
                            "Filtering by tag: "
                            <span class="active-tag">{tag}</span>
                            <a href="/" class="clear-filter" title="Clear filter">X</a>
                        </p>
                    </div>
                })
            }}

            // Use the existing working component with filtered projects
            <ProjectSearch projects={filtered_projects()} />
        </div>
    }
}
