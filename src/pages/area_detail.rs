use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use crate::model::{JDArea, JDCategory};
use crate::components::{JohnnyDecimalRange, CategoryBadge};

#[component]
pub fn AreaDetailPage() -> impl IntoView {
    let areas_signal = use_context::<ReadSignal<Vec<JDArea>>>()
        .expect("Areas context not found!");

    let categories_signal = use_context::<ReadSignal<Vec<JDCategory>>>()
        .expect("Categories context not found!");

    // Get the area ID from URL params
    let params = use_params_map();
    let area_id = move || {
        params.with(|p| {
            p.get("id")
                .and_then(|id| id.parse::<i64>().ok())
                .unwrap_or(0)
        })
    };

    // Find the area
    let current_area = move || {
        let id = area_id();
        areas_signal.get().iter().find(|a| a.id == id).cloned()
    };

    // Get categories for this area
    let area_categories = move || {
        let id = area_id();
        categories_signal.get().iter()
            .filter(|c| c.area_id == id)
            .cloned()
            .collect::<Vec<_>>()
    };

    view! {
        <Show
            when=move || current_area().is_some()
            fallback=|| view! {
                <div class="not-found container">
                    <h2>"Area not found!"</h2>
                </div>
            }
        >
            {move || {
                let area = current_area().unwrap();
                let start = area.id.to_string();
                let end = (area.id + 9).to_string();

                view! {
                    <div class="area-detail container">
                        <Title text={format!("{} - Tyler Harpool", area.name.clone())}/>

                        <header class="area-header">
                            <div class="breadcrumbs">
                                <a href="/areas">"Areas"</a>
                                " > "
                                <span class="current">{area.name.clone()}</span>
                            </div>

                            <div class="area-title-section">
                                <JohnnyDecimalRange
                                    start=start
                                    end=end
                                    name=area.name.clone()
                                />
                            </div>

                            <p class="area-description">{area.description.clone()}</p>
                        </header>

                        <h2 class="section-title">"Categories in this Area"</h2>
                        <div class="jd-categories-grid">
                            {move || area_categories().into_iter().map(|category| {
                                let category_id = category.id.to_string();

                                view! {
                                    <div class="jd-category-card">
                                        <div class="jd-category-header">
                                            <div class="jd-category-label">
                                                <CategoryBadge id=category_id.clone() />
                                            </div>
                                            <div class="jd-category-title-container">
                                                <a href={format!("/categories/{}", category.id)} class="jd-category-title">
                                                    {category.name.clone()}
                                                </a>
                                            </div>
                                        </div>
                                        <div class="jd-category-description">
                                            {category.description.clone()}
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>

                        <div class="navigation-links">
                            <a href="/areas" class="btn btn-back">"← Back to All Areas"</a>
                        </div>
                    </div>
                }
            }}
        </Show>
    }
}
