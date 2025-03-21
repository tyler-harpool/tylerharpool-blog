use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use crate::model::{JDArea, JDCategory, Project};
use crate::utils::format::format_date;

#[component]
pub fn CategoryDetailPage() -> impl IntoView {
    let categories_signal = use_context::<ReadSignal<Vec<JDCategory>>>()
        .expect("Categories context not found!");

    let areas_signal = use_context::<ReadSignal<Vec<JDArea>>>()
        .expect("Areas context not found!");

    let projects_signal = use_context::<ReadSignal<Vec<Project>>>()
        .expect("Projects context not found!");

    // Get the category ID from URL params
    let params = use_params_map();
    let category_id = move || {
        params.with(|p| {
            p.get("id")
                .and_then(|id| id.parse::<u8>().ok())
                .unwrap_or(0)
        })
    };

    // Find the category
    let current_category = move || {
        let id = category_id();
        categories_signal.get().iter().find(|c| c.id == id).cloned()
    };

    // Find the parent area
    let parent_area = move || {
        current_category().and_then(|cat| {
            areas_signal.get().iter().find(|a| a.id == cat.area_id).cloned()
        })
    };

    // Get projects for this category
    let category_projects = move || {
        let cat_id = category_id();
        projects_signal.get().iter()
            .filter(|p| p.jd_category.as_ref().map_or(false, |c| c.id == cat_id))
            .cloned()
            .collect::<Vec<_>>()
    };

    // Function to assign decimal IDs to articles within a category
    let get_article_decimal_id = |index: usize, category_id: u8| -> String {
        format!("{}.{}", category_id, index + 1)
    };

    view! {
        <Show
            when=move || current_category().is_some()
            fallback=|| view! {
                <div class="not-found container">
                    <h2>"Category not found!"</h2>
                </div>
            }
        >
            {move || {
                let category = current_category().unwrap();
                let area = parent_area().unwrap_or_else(|| JDArea {
                    id: 0,
                    name: "Unknown Area".into(),
                    description: "".into(),
                });

                view! {
                    <div class="category-detail container">
                        <Title text={format!("{} - Tyler Harpool", category.name.clone())}/>

                        <header class="category-header">
                            <div class="breadcrumbs">
                                <a href="/areas">"Areas"</a>
                                " > "
                                <a href={format!("/areas/{}", area.id)}>
                                    <span class="area-code">{format!("{}-{}", area.id, area.id + 9)}</span>
                                    " "{area.name.clone()}
                                </a>
                                " > "
                                <span class="current">{category.name.clone()}</span>
                            </div>

                            <div class="category-title-section">
                                <span class="category-number">{category.id}</span>
                                <h1>{category.name.clone()}</h1>
                            </div>

                            <p class="category-description">{category.description.clone()}</p>
                        </header>

                        <h2>"Posts in this Category"</h2>

                        <Show
                            when=move || !category_projects().is_empty()
                            fallback=|| view! {
                                <div class="empty-state">
                                    <p>"No posts found in this category yet. Check back later!"</p>
                                </div>
                            }
                        >
                        <div class="jd-decimal-list">
                            {move || {
                                let mut projects = category_projects();
                                projects.sort_by(|a, b| b.created_at.cmp(&a.created_at));

                                projects.into_iter().enumerate().map(|(index, project)| {
                                    let formatted_date = format_date(project.created_at);
                                    let decimal_id = get_article_decimal_id(index, category.id);

                                    view! {
                                        <div class="decimal-article">
                                            <div class="article-header">
                                                <div class="decimal-container">
                                                    <span class="decimal-number">{decimal_id}</span>
                                                </div>
                                                <div class="article-title-container">
                                                    <a href={format!("/projects/{}", project.slug)} class="article-title">
                                                        {project.title}
                                                    </a>
                                                </div>
                                            </div>

                                            <div class="article-content">
                                                <p class="article-summary">{project.summary.clone()}</p>
                                                <div class="article-meta">
                                                    <span class="article-date">{formatted_date}</span>
                                                    <div class="article-tags">
                                                        {project.tech_stack.iter().map(|tech| {
                                                            view! {
                                                                <span class="article-tag">{tech.clone()}</span>
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

                        <div class="navigation-links">
                            <a href={format!("/areas/{}", area.id)} class="btn btn-back">
                                {format!("← Back to {}", area.name)}
                            </a>
                        </div>
                    </div>
                }
            }}
        </Show>
    }
}
