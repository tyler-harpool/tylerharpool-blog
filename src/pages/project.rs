use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use crate::model::{Project, JDArea};
use crate::utils::{format::format_date, markdown::markdown_to_html};
use crate::components::RenderRelatedProjects;

#[component]
pub fn ProjectPage() -> impl IntoView {
    // Grab the project list from context
    let projects_signal = use_context::<ReadSignal<Vec<Project>>>()
        .expect("Projects context not found in ProjectPage!");

    // The :slug from the URL
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());

    // Find the project that matches
    let current_project = move || {
        let s = slug();
        projects_signal
            .get()
            .iter()
            .find(|p| p.slug == s)
            .cloned()
    };

    // Format creation date
    let formatted_date = move || {
        current_project().map(|proj| {
            format_date(proj.created_at)
        })
        .unwrap_or_else(|| "Unknown".to_string())
    };

    // Convert current project's content to HTML
    let content_html = move || {
        current_project().map(|proj| markdown_to_html(&proj.content)).unwrap_or_default()
    };

    // Show the project or a fallback if none found
    view! {
        <Show
            when=move || current_project().is_some()
            fallback=|| view! {
                <div class="not-found container">
                    <h2>"Project not found!"</h2>
                </div>
            }
        >
            {move || {
                let project = current_project().unwrap();
                let title = project.title.clone();

                view! {
                    <div class="project-detail container">
                        <Title text={format!("{} - Tyler Harpool", title)}/>

                        <header class="project-header">
                            {project.jd_category.as_ref().map(|cat| {
                                let areas_signal = use_context::<ReadSignal<Vec<JDArea>>>()
                                    .expect("Areas context not found!");

                                let parent_area = areas_signal.get().iter()
                                    .find(|a| a.id == cat.area_id)
                                    .cloned();

                                let project_decimal = format!("{}.{}", cat.id, project.id.unwrap_or(0));

                                view! {
                                  <div class="project-jd-info">
                                      <div class="project-breadcrumbs">
                                          <a href="/areas">"Areas"</a>
                                          " > "
                                          {parent_area.as_ref().map(|area| view! {
                                              <>
                                                  <a href={format!("/areas/{}", area.id)}>
                                                      <span class="breadcrumb-area-code">{format!("{}-{}", area.id, area.id + 9)}</span>
                                                      <span class="breadcrumb-area-name">{" "}{area.name.clone()}</span>
                                                  </a>
                                                  " > "
                                              </>
                                          })}
                                          <a href={format!("/categories/{}", cat.id)}>
                                              <span class="breadcrumb-category-code">{cat.id}</span>
                                              <span class="breadcrumb-category-name">{" "}{cat.name.clone()}</span>
                                          </a>
                                      </div>

                                      <div class="project-decimal-container">
                                          <span class="project-decimal">{project_decimal}</span>
                                          <div class="project-category-label">
                                              <span class="project-category-id">{cat.id}</span>
                                              <span class="project-category-name">{cat.name.clone()}</span>
                                          </div>
                                      </div>
                                  </div>
                                }
                            })}

                            <h1 class="project-title">{title}</h1>
                            <p class="date">"Published on " {formatted_date()}</p>

                            <div class="project-meta">
                                <div class="tech-stack">
                                    <h3>"Technologies Used"</h3>
                                    <ul class="tags">
                                        {project.tech_stack.iter().map(|tech| {
                                           let tech_str = tech.clone();
                                            view! {
                                                <li class="tag">{tech_str}</li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </div>

                                <div class="project-links">
                                    {project.repo_url.clone().map(|url| view! {
                                        <a href={url} class="btn btn-primary" target="_blank" rel="noopener noreferrer">
                                            "View Code Repository"
                                        </a>
                                    })}
                                    {project.live_url.clone().map(|url| view! {
                                        <a href={url} class="btn btn-secondary" target="_blank" rel="noopener noreferrer">
                                            "Visit Live Site"
                                        </a>
                                    })}
                                </div>
                            </div>
                        </header>

                        {project.thumbnail.clone().map(|url| view! {
                            <div class="project-image">
                                <img src={url} alt={project.title.clone()} />
                            </div>
                        })}

                        <div class="project-summary">
                            <h2>"Project Summary"</h2>
                            <p>{project.summary.clone()}</p>
                        </div>

                        <div class="project-content">
                            <div inner_html={content_html()}></div>
                        </div>

                        <footer class="project-footer">
                            {project.jd_category.as_ref().map(|cat| {

                                // Store the category information in local variables
                                let category_id = cat.id;
                                let category_name = cat.name.clone();

                                let category_link = format!("/categories/{}", category_id);
                                let view_all_text = format!("View all in {}", category_name);

                                view! {
                                    <div class="related-projects-section">
                                        <RenderRelatedProjects
                                            project_id={project.id}
                                            category_id={category_id}
                                            category_link={category_link}
                                            view_all_text={view_all_text}
                                        />
                                    </div>
                                }
                            })}

                            <a href="/" class="btn btn-back">"← Back to All Articles"</a>
                        </footer>
                    </div>
                }
            }}
        </Show>
    }
}
