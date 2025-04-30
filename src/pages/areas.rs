use crate::{JDArea, JDCategory};  // Ensure correct imports for JDArea and JDCategory
use leptos::prelude::*;
use leptos_meta::Title;  // Import Title for setting the document title
use std::path::Path;  // Import Path for file handling
use std::fs;
use leptos::logging::log;

#[component]
pub fn AreasPage() -> impl IntoView {
  let areas_signal = use_context::<ReadSignal<Vec<JDArea>>>()
      .expect("Areas context not found!");

  let categories_signal = use_context::<ReadSignal<Vec<JDCategory>>>()
      .expect("Categories context not found!");

    // Assuming you have a context for categories as well
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
                    // Fetch description from README file
                    let area_description = area.description.clone();

                    // For each area, filter the associated categories
                    let area_categories = categories_signal.get().iter()
                        .filter(|c| c.area_id == area.id)
                        .cloned()
                        .collect::<Vec<_>>();

                    view! {
                        <div class="jd-area-card">
                            <div class="jd-area-header">
                                <div class="jd-area-label">
                                    <span class="jd-area-range">{format!("{}-{}", area.id, area.id + 9)}</span>
                                </div>
                                <div class="jd-area-title-container">
                                    <a href={format!("/areas/{}", area.id)} class="jd-area-title">
                                        {area.name.clone()}
                                    </a>
                                </div>
                            </div>

                            <div class="jd-area-description">
                                {area_description}
                            </div>

                            <div class="jd-categories-list">
                                {area_categories.into_iter().map(|category| {
                                    view! {
                                        <div class="jd-category-row">
                                            <a href={format!("/categories/{}", category.id)} class="jd-category-link">
                                                <span class="jd-category-number">{category.id}</span>
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
