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

    // Get the current project (needed for related_articles)
    let current_project = move || {
        project_id.and_then(|id| {
            projects_signal.get().iter().find(|p| p.id == Some(id)).cloned()
        })
    };
    fn find_similar_projects(
        project: &Project,
        all_projects: &[Project],
        limit: usize
    ) -> Vec<Project> {
        // Skip if no tags
        if project.tech_stack.is_empty() {
            return Vec::new();
        }

        // Calculate similarity scores
        let mut scored_projects = all_projects.iter()
            .filter(|p| p.id != project.id) // Not the same project
            .map(|other| {
                // Count matching tags
                let common_tags = project.tech_stack.iter()
                    .filter(|tag| other.tech_stack.contains(tag))
                    .count();

                // Simple score: number of matching tags
                let score = common_tags;

                (score, other.clone())
            })
            .filter(|(score, _)| *score > 0) // Must have at least 1 tag in common
            .collect::<Vec<_>>();

        // Sort by score, descending
        scored_projects.sort_by(|(score_a, _), (score_b, _)| score_b.cmp(score_a));

        // Take the top N results
        scored_projects.into_iter()
            .take(limit)
            .map(|(_, p)| p)
            .collect()
    }
    // Find related projects - either by ID or by category
    let related_projects = move || {
        let mut related = Vec::new();

        // First use manually specified related articles
        if let Some(project) = current_project() {
            for related_id in &project.related_articles {
                // Try to find the project by its JD identifier
                if let Some(related_project) = projects_signal.get().iter()
                    .find(|p| p.jd_identifier == *related_id) {
                    related.push(related_project.clone());
                }
            }

            // If we still need more related articles, auto-generate them
            if related.len() < 3 {
                let auto_related = find_similar_projects(
                    &project,
                    &projects_signal.get(),
                    3 - related.len()
                );

                // Only add ones not already included
                let existing_ids: Vec<Option<i64>> = related.iter().map(|p| p.id).collect();
                for p in auto_related {
                    if !existing_ids.contains(&p.id) {
                        related.push(p);
                    }
                }
            }
        }

        // Fill remaining slots with category-based projects if needed
        if related.len() < 3 {
            let existing_ids: Vec<Option<i64>> = related.iter().map(|p| p.id).collect();

            let category_projects = projects_signal.get().iter()
                .filter(|p| p.id != project_id &&
                          !existing_ids.contains(&p.id) &&
                          p.jd_category.as_ref().map_or(false, |c| c.id == category_id))
                .take(3 - related.len())
                .cloned()
                .collect::<Vec<_>>();

            related.extend(category_projects);
        }

        related
    };

    // Check if there are related projects
    let has_related = move || !related_projects().is_empty();

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
                <h3>"Related Articles"</h3>
                <ul>
                    {move ||
                        related_projects().into_iter()
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
