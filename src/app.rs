use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_router::path;
use leptos_router::hooks::use_params_map;
use std::time::SystemTime;
use leptos::logging::log;
use crate::model::Project;
// This is your main "shell" function
// It sets up the <html>, <head>, <body> structure,
// then calls <App/>.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="Tyler Harpool's personal blog and project showcase"/>
                <meta name="keywords" content="web development, software engineering, projects, blog"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options islands=true/>
                <MetaTags/>
                <link rel="stylesheet" id="leptos" href="/pkg/tylerharpool-blog.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
            </head>
            <body>
                // Our top-level App
                <App/>
            </body>
        </html>
    }
}

// -----------------------------------------
// 2) Main App that sets up shared context + routes
// -----------------------------------------
#[component]
pub fn App() -> impl IntoView {
    // Provide context for metadata
    provide_meta_context();

    // -- Create and provide a single list of Projects --
    let projects = vec![
        Project {
            id: Some(1),
            title: "Building a Modern Web App with Leptos and Rust".into(),
            slug: "building-with-leptos".into(),
            summary: "An exploration of Rust's web framework ecosystem and how Leptos is pushing the boundaries.".into(),
            content: "# Project Details\n\nThis project was built using Rust and Leptos...".into(),
            tech_stack: vec!["Rust".into(), "Leptos".into(), "WebAssembly".into()],
            repo_url: Some("https://github.com/tylerharpool/building-with-leptos".into()),
            live_url: Some("https://building-with-leptos.tylerharpool.com".into()),
            thumbnail: None,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        },
        Project {
            id: Some(2),
            title: "Implementing Islands Architecture in a Rust Web Framework".into(),
            slug: "islands-architecture".into(),
            summary: "How partial hydration can improve performance while maintaining interactivity.".into(),
            content: "# Project Details\n\nThis project highlights how Islands Architecture...".into(),
            tech_stack: vec!["Rust".into(), "Leptos".into(), "Islands".into()],
            repo_url: Some("https://github.com/tylerharpool/islands-architecture".into()),
            live_url: Some("https://islands-architecture.tylerharpool.com".into()),
            thumbnail: None,
            // ~1 week ago
            created_at: SystemTime::now()
                .checked_sub(std::time::Duration::from_secs(7 * 24 * 3600))
                .unwrap_or(SystemTime::now()),
            updated_at: SystemTime::now(),
        },
        Project {
            id: Some(3),
            title: "Server Functions: Bridging the Frontend-Backend Divide".into(),
            slug: "server-functions".into(),
            summary: "Using Rust on both ends of the stack to create a seamless development experience.".into(),
            content: "# Project Details\n\nThis project demonstrates how server functions can unify front and back end...".into(),
            tech_stack: vec!["Rust".into(), "Leptos".into(), "Axum".into()],
            repo_url: Some("https://github.com/tylerharpool/server-functions".into()),
            live_url: Some("https://server-functions.tylerharpool.com".into()),
            thumbnail: None,
            // ~2 weeks ago
            created_at: SystemTime::now()
                .checked_sub(std::time::Duration::from_secs(14 * 24 * 3600))
                .unwrap_or(SystemTime::now()),
            updated_at: SystemTime::now(),
        },
    ];

    let (projects_signal, _set_projects) = signal(projects.clone());
    provide_context(projects_signal);

    view! {
        <Stylesheet id="leptos" href="/pkg/tylerharpool-blog.css"/>
        <Title text="Welcome to Leptos"/>
        <Header/>
        // Router for our routes
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                <Route
                    path=StaticSegment("")
                    view=move || {
                        let projects = projects_signal.get().clone();
                        view! { <HomePage projects=projects /> }
                    }
                />
                    <Route path=StaticSegment("/about") view=AboutPage/>
                    // Project detail route
                    <Route path=path!("/projects/:slug") view=ProjectPage/>
                </Routes>
            </main>
        </Router>
    }
}

// -----------------------------------------
// 3) HomePage: Display list of all projects
// -----------------------------------------
// 1) Mark this component as an "island"
#[component]
fn HomePage(projects: Vec<Project>) -> impl IntoView {
    let (projects_signal, _) = signal(projects);

    // Format date helper function
    let format_date = |date: SystemTime| -> String {
        let secs = date.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
        let days = (secs / 86400) % 30;
        let months = (secs / 2592000) % 12;
        let years = secs / 31104000;
        format!("{}-{:02}-{:02}", years + 1970, months + 1, days + 1)
    };

    view! {
        <div class="container">
            <h1>"Tyler Harpool's Blog"</h1>
            <p class="intro-text">
                "Welcome to my blog where I share my thoughts and experiences on software architecture,
                enterprise solutions, and emerging technologies like Rust and WebAssembly."
            </p>

            // Include the search island component
            <ProjectSearch projects={projects_signal.get()}/>
        </div>
    }
}


#[island]
fn ProjectSearch(projects: Vec<Project>) -> impl IntoView {
    let (projects_signal, _) = signal(projects);
    let (search_query, set_search_query) = signal(String::new());

    let filtered_projects = move || {
        let mut projects = projects_signal.get().clone();
        projects.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let q = search_query.get().to_lowercase();

        if q.is_empty() {
            return projects;
        }

        projects
            .into_iter()
            .filter(|p| {
                p.title.to_lowercase().contains(&q) ||
                p.summary.to_lowercase().contains(&q) ||
                p.tech_stack.iter().any(|tech| tech.to_lowercase().contains(&q))
            })
            .collect::<Vec<_>>()
    };

    // Format date helper function
    let format_date = |date: SystemTime| -> String {
        let secs = date.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
        let days = (secs / 86400) % 30;
        let months = (secs / 2592000) % 12;
        let years = secs / 31104000;
        format!("{}-{:02}-{:02}", years + 1970, months + 1, days + 1)
    };

    view! {
        <div>
        <input
            type="text"
            placeholder="Search projects..."
            class="search-input"
            on:input=move |ev| {
                log!("User typed: {}", event_target_value(&ev));
                set_search_query(event_target_value(&ev));
            }
        />

            <div class="blog-list">
                {move || filtered_projects().into_iter().map(|project| {
                    let formatted_date = format_date(project.created_at);
                    view! {
                        <div class="blog-item">
                            <span class="blog-date">{formatted_date}</span>
                            <div class="blog-title">
                                <a href={format!("/projects/{}", project.slug)}>
                                    {project.title}
                                </a>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

// -----------------------------------------
// 4) ProjectPage: Display details for one project
// -----------------------------------------
// ------------------------------------------
// 4) ProjectPage: detail view
// ------------------------------------------
#[component]
fn ProjectPage() -> impl IntoView {
    // Grab the same project list from context
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
            let secs = proj.created_at.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
            let days = (secs / 86400) % 30;
            let months = (secs / 2592000) % 12;
            let years = secs / 31104000;
            format!("{}-{:02}-{:02}", years + 1970, months + 1, days + 1)
        })
        .unwrap_or_else(|| "Unknown".to_string())
    };

    // Naive Markdown→HTML function
    fn markdown_to_html(content: &str) -> String {
        let mut html = String::new();
        for line in content.lines() {
            if line.starts_with("# ") {
                html.push_str(&format!("<h1>{}</h1>", &line[2..]));
            } else if line.starts_with("## ") {
                html.push_str(&format!("<h2>{}</h2>", &line[3..]));
            } else if line.starts_with("- ") {
                html.push_str(&format!("<li>{}</li>", &line[2..]));
            } else if line.is_empty() {
                html.push_str("<br />");
            } else {
                html.push_str(&format!("<p>{}</p>", line));
            }
        }
        html
    }

    // Convert current project's content to HTML
    let content_html = move || {
        current_project().map(|proj| markdown_to_html(&proj.content)).unwrap_or_default()
    };

    // Show the project or a fallback if none found
    view! {
        <Show
            when=move || current_project().is_some()
            fallback=|| view! {
                <div class="not-found">
                    <h2>"Project not found!"</h2>
                </div>
            }
        >
            {move || {
                let project = current_project().unwrap();
                view! {
                    <div class="project-detail">
                        <header class="project-header">
                            <h1>{project.title.clone()}</h1>
                            <p class="date">"Published on " {formatted_date}</p>

                            <div class="project-meta">
                                <div class="tech-stack">
                                    <h3>"Technologies Used"</h3>
                                    <ul class="tags">
                                        // Clone or to_string() so it renders properly
                                        {project.tech_stack.iter().map(|tech| {
                                            view! {
                                                <li class="tag">{tech.clone()}</li>
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
                            <div inner_html={content_html}></div>
                        </div>

                        <footer class="project-footer">
                            <a href="/" class="btn btn-back">"← Back to Projects"</a>
                        </footer>
                    </div>
                }
            }}
        </Show>
    }
}

// -----------------------------------------
// 5) About Page (unchanged)
// -----------------------------------------
#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="about-page container">
            <Title text="About - Tyler Harpool"/>

            <div class="about-header">
                <h1>"About Tyler Harpool"</h1>
                <div class="about-image">
                    <img src="/images/profile.png" alt="Tyler Harpool"/>
                </div>
            </div>

            <div class="about-content">
                <section class="bio">
                    <p>
                        "My journey in technology began early—all the way back in 5th grade when I wrote my first lines of code.
                        That childhood curiosity evolved into a lifelong passion for building and architecting technology that solves real problems."
                    </p>
                    <p>
                        "Today, I'm an Enterprise Solutions Architect, designing and implementing comprehensive technology strategies for organizations.
                        Prior to this role, I worked as a software engineer, developing applications and systems from the ground up.
                        This hands-on experience gives me a unique perspective when architecting enterprise-level solutions,
                        as I understand the technical challenges and considerations at every level of implementation."
                    </p>
                    <p>
                        "In my current role, I bridge the gap between business requirements and technological capabilities,
                        creating scalable, efficient architectures that meet both immediate needs and allow for future growth.
                        I particularly enjoy working with emerging technologies like Rust and exploring frameworks such as Leptos
                        to push the boundaries of what's possible in enterprise applications."
                    </p>
                </section>

                <section class="skills">
                    <h2>"Technical Expertise"</h2>
                    <div class="skills-grid">
                        <div class="skill-category">
                            <h3>"Architecture"</h3>
                            <ul>
                                <li>"System design"</li>
                                <li>"Microservices"</li>
                                <li>"Cloud architecture"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Languages"</h3>
                            <ul>
                                <li>"Rust"</li>
                                <li>"JavaScript/TypeScript"</li>
                                <li>"Python"</li>
                                <li>"Go"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Frontend"</h3>
                            <ul>
                                <li>"Leptos"</li>
                                <li>"React"</li>
                                <li>"Vue.js"</li>
                                <li>"HTML/CSS"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Backend"</h3>
                            <ul>
                                <li>"Axum"</li>
                                <li>"Node.js"</li>
                                <li>"Django"</li>
                                <li>"Express"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Databases"</h3>
                            <ul>
                                <li>"PostgreSQL"</li>
                                <li>"MongoDB"</li>
                                <li>"SQLite"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Cloud & DevOps"</h3>
                            <ul>
                                <li>"AWS"</li>
                                <li>"Azure"</li>
                                <li>"Docker"</li>
                                <li>"Kubernetes"</li>
                                <li>"CI/CD pipelines"</li>
                            </ul>
                        </div>
                    </div>
                </section>

                <section class="closing">
                    <p>
                        "When I'm not designing solutions, I enjoy contributing to open-source projects,
                        writing technical articles, and mentoring aspiring developers. I believe in creating
                        technology that is not only functional but also accessible and user-friendly."
                    </p>

                    <p>
                        "Feel free to explore my projects or reach out if you'd like to discuss technology
                        strategy or collaborate on something exciting!"
                    </p>
                </section>

                <section class="contact">
                    <h2>"Get in Touch"</h2>
                    <div class="social-links">
                        <a href="https://github.com/tylerharpool" target="_blank" rel="noopener noreferrer" class="social-link">
                            <span class="social-icon">_github</span>
                            "GitHub"
                        </a>
                        <a href="https://linkedin.com/in/tylerharpool" target="_blank" rel="noopener noreferrer" class="social-link">
                            <span class="social-icon">_linkedin</span>
                            "LinkedIn"
                        </a>
                        <a href="mailto:contact@tylerharpool.com" class="social-link">
                            <span class="social-icon">_email</span>
                            "Email"
                        </a>
                    </div>
                </section>
            </div>
        </div>
    }
}

// -----------------------------------------
// 6) Header
// -----------------------------------------
#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <nav>
                <a href="/">"Home"</a>
                <a href="/about">"About"</a>
            </nav>
        </header>
    }
}
