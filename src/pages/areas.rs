use leptos::prelude::*;
use leptos_meta::Title;
use crate::model::{JDArea, JDCategory};
use crate::components::{JohnnyDecimalRange, CategoryBadge};

#[component]
pub fn AreasPage() -> impl IntoView {
    let areas_signal = use_context::<ReadSignal<Vec<JDArea>>>()
        .expect("Areas context not found!");

    // Get the categories context
    let categories_signal = use_context::<ReadSignal<Vec<JDCategory>>>()
        .expect("Categories context not found!");

    view! {
        <div class="container">
            <Title text="Browse by Area - Tyler Harpool"/>
            <h1>"Browse by Area"</h1>
            <p class="intro-text">
                "This blog is organized using the Johnny Decimal system, which groups content into meaningful areas and categories."
            </p>

            <div class="jd-areas">
                {move || areas_signal.get().clone().into_iter().map(|area| {
                    // For each area, filter the associated categories
                    let area_categories = categories_signal.get().iter()
                        .filter(|c| c.area_id == area.id)
                        .cloned()
                        .collect::<Vec<_>>();

                    // Get range
                    let start = area.id.to_string();
                    let end = (area.id + 9).to_string();

                    view! {
                        <div class="jd-area-card">
                            <div class="jd-area-header">
                                <JohnnyDecimalRange
                                    start=start
                                    end=end
                                    name=area.name.clone()
                                />
                            </div>

                            <div class="jd-area-description">
                                {area.description.clone()}
                            </div>

                            <div class="jd-categories-list">
                                {area_categories.into_iter().map(|category| {
                                    let category_id = category.id.to_string();

                                    view! {
                                        <div class="jd-category-row">
                                            <a href={format!("/categories/{}", category.id)} class="jd-category-link">
                                                <CategoryBadge id=category_id />
                                                <span class="jd-category-name">{category.name}</span>
                                            </a>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
