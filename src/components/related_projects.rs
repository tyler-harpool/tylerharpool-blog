use leptos::prelude::*;
use crate::model::Project;

#[component]
pub fn RenderRelatedProjects(
    project_id: Option<i64>,
    category_id: u8,
    category_link: String,
    view_all_text: String,
) -> impl IntoView {
    let projects_signal = use_context::<ReadSignal<Vec<Project>>>()
        .unwrap_or_else(|| {
            panic!("Projects context not found! Please ensure the context is provided.")
        });

    // Check if there are related projects
    let has_related = move || {
        projects_signal.get().iter()
            .filter(|p| p.id != project_id &&
                       p.jd_category.as_ref().map_or(false, |c| c.id == category_id))
            .count() > 0
    };

    view! {
        <Show
            when=has_related
            fallback=|| view! {
                <div class="no-related-content" aria-live="polite">
                    <p>"No related articles found in this category."</p>
                </div>
            }
        >
            <div class="related-projects" aria-label="Related Projects">
                <h3>"Related Articles in this Category"</h3>
                <ul>
                    {move ||
                        projects_signal.get().iter()
                            .filter(|p| p.id != project_id &&
                                      p.jd_category.as_ref().map_or(false, |c| c.id == category_id))
                            .take(3)
                            .map(|p| view! {
                                <li>
                                    <a href={format!("/projects/{}", p.slug)} aria-label={format!("Read more about {}", p.title)}>
                                        {p.title.clone()}
                                    </a>
                                </li>
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
                <a href={category_link.clone()} class="view-more" aria-label="View more related projects">
                    {view_all_text.clone()}
                </a>
            </div>
        </Show>
    }
}
