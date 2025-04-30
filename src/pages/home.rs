use leptos::prelude::*;
use crate::model::Project;
use crate::components::TaggedSearch;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;

#[component]
pub fn HomePage(projects: Vec<Project>) -> impl IntoView {
    let (projects_signal, _) = signal(projects.clone());
    let (current_page, set_current_page) = signal(1);
    let items_per_page = 6;

    let query = use_query_map();
    let tag_filter = move || {
        query.with(|q| q.get("tag").map(|s| s.clone()).unwrap_or_default())
    };
    // Get most recent posts
    let recent_posts = move || {
        let mut all = projects_signal.get().clone();
        all.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        all
    };

    // Pagination for recent posts
    let total_recent_pages = move || {
        let total = recent_posts().len();
        if total == 0 { 1 } else { (total + items_per_page - 1) / items_per_page }
    };

    let current_page_items = move || {
        let items = recent_posts();
        let page = current_page.get();
        let start = (page - 1) * items_per_page;

        items.into_iter()
            .skip(start)
            .take(items_per_page)
            .collect::<Vec<_>>()
    };

    // Navigation functions
    let next_page = move || {
        let current = current_page.get();
        let max = total_recent_pages();
        if current < max {
            set_current_page(current + 1);
        }
    };

    let prev_page = move || {
        let current = current_page.get();
        if current > 1 {
            set_current_page(current - 1);
        }
    };

    let go_to_page = move |page: usize| {
        set_current_page(page);
    };

    // Boolean conditions for Show components
    let has_recent_posts = move || !current_page_items().is_empty();
    let should_show_pagination = move || total_recent_pages() > 1;

    view! {
        <div class="container">
            <Title text="Tyler Harpool - Technology & Government Blog"/>
            <h1>"Tyler Harpool's Blog"</h1>
            <p class="intro-text">
                "Welcome to my blog where I share my thoughts on software architecture,
                enterprise solutions, and emerging technologies."
            </p>

            <div class="search-box">
            <TaggedSearch
                projects={projects.clone()}
                active_tag={tag_filter()}
            />
            </div>


        </div>
    }
}
