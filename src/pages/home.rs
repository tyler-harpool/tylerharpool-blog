use leptos::prelude::*;
use crate::model::Project;
use crate::components::ProjectSearch;
use leptos_meta::Title;
#[component]
pub fn HomePage(projects: Vec<Project>) -> impl IntoView {
    // let (projects_signal, _) = signal(projects);

    view! {
        <div class="container">
            <Title text="Tyler Harpool - Technology & Government Blog"/>
            <h1>"Tyler Harpool's Blog"</h1>
            <p class="intro-text">
                "Welcome to my blog where I share my thoughts and experiences on software architecture,
                enterprise solutions, and emerging technologies like Rust and WebAssembly."
            </p>

            <div class="jd-system-section">
                <div class="recent-posts-section">
                    <h2>"Recent Posts"</h2>
                    <ProjectSearch projects={projects}/>
                </div>
            </div>
        </div>
    }
}
