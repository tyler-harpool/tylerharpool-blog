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
use crate::model::{
    JDArea,
    JDCategory,
    get_all_areas,
    get_all_categories,
    find_category_by_id,
    find_area_by_id,
    get_categories_for_area
};
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

    // -- Create and provide JD Areas and Categories --
    let areas = get_all_areas();
    let categories = get_all_categories();

    let (areas_signal, _) = signal(areas.clone());
    let (categories_signal, _) = signal(categories.clone());

    provide_context(areas_signal);
    provide_context(categories_signal);

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
            jd_category: find_category_by_id(12), // Web Frameworks
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
            jd_category: find_category_by_id(13), // Software Architecture
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
            jd_category: find_category_by_id(12), // Web Frameworks
        },
        // Adding a few more projects with different categories
        Project {
            id: Some(4),
            title: "Cloud-Native Deployment Strategies".into(),
            slug: "cloud-native-deployment".into(),
            summary: "Best practices for deploying applications in cloud environments with minimal downtime.".into(),
            content: "# Cloud-Native Deployment\n\nThis article explores various strategies...".into(),
            tech_stack: vec!["AWS".into(), "Docker".into(), "Kubernetes".into()],
            repo_url: None,
            live_url: None,
            thumbnail: None,
            created_at: SystemTime::now()
                .checked_sub(std::time::Duration::from_secs(3 * 24 * 3600))
                .unwrap_or(SystemTime::now()),
            updated_at: SystemTime::now(),
            jd_category: find_category_by_id(23), // Containerization
        },
        Project {
            id: Some(5),
            title: "Government Digital Transformation Initiatives".into(),
            slug: "gov-digital-transformation".into(),
            summary: "How governments are leveraging technology to improve service delivery and citizen engagement.".into(),
            content: "# Government Digital Transformation\n\nThis article examines recent initiatives...".into(),
            tech_stack: vec!["GovTech".into(), "Digital Services".into()],
            repo_url: None,
            live_url: None,
            thumbnail: None,
            created_at: SystemTime::now()
                .checked_sub(std::time::Duration::from_secs(10 * 24 * 3600))
                .unwrap_or(SystemTime::now()),
            updated_at: SystemTime::now(),
            jd_category: find_category_by_id(31), // GovTech Initiatives
        },
    ];

    let (projects_signal, _set_projects) = signal(projects.clone());
    provide_context(projects_signal);

    view! {
        <Stylesheet id="leptos" href="/pkg/tylerharpool-blog.css"/>
        <Title text="Tyler Harpool - Technology & Government Blog"/>
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
                    // New routes for Johnny Decimal browsing
                    <Route path=path!("/areas") view=AreasPage/>
                    <Route path=path!("/areas/:id") view=AreaDetailPage/>
                    <Route path=path!("/categories/:id") view=CategoryDetailPage/>
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

    // Get areas and categories
    let areas_signal = use_context::<ReadSignal<Vec<JDArea>>>()
        .expect("Areas context not found!");

    let categories_signal = use_context::<ReadSignal<Vec<JDCategory>>>()
        .expect("Categories context not found!");

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
                <ProjectSearch projects={projects_signal.get()}/>
            </div>
           </div>
        </div>
    }
}


#[island]
fn ProjectSearch(projects: Vec<Project>) -> impl IntoView {
    let (projects_signal, _) = signal(projects);
    let (search_query, set_search_query) = signal(String::new());

    // Filtered projects implementation
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
                p.tech_stack.iter().any(|tech| tech.to_lowercase().contains(&q)) ||
                p.jd_category.as_ref().map_or(false, |c|
                    c.name.to_lowercase().contains(&q) ||
                    c.description.to_lowercase().contains(&q)
                )
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

            <div class="search-results">
                {move || filtered_projects().into_iter().map(|project| {
                    let formatted_date = format_date(project.created_at);
                    let decimal_id = project.jd_category.as_ref().map_or("".to_string(), |cat| {
                        format!("{}.{}", cat.id, project.id.unwrap_or(0))
                    });

                    view! {
                        <div class="search-result-item">
                            <div class="result-header">
                                {project.jd_category.as_ref().map(|_| view! {
                                    <div class="result-decimal-container">
                                        <span class="result-decimal">{decimal_id}</span>
                                    </div>
                                })}

                                <div class="result-title-container">
                                    <a href={format!("/projects/{}", project.slug)} class="result-title">
                                        {project.title}
                                    </a>
                                </div>
                            </div>

                            <div class="result-content">
                                <p class="result-summary">{project.summary.clone()}</p>
                                <div class="result-meta">
                                    <span class="result-date">{formatted_date}</span>
                                    <div class="result-tags">
                                        {project.tech_stack.iter().map(|tech| {
                                            view! {
                                                <span class="result-tag">{tech.clone()}</span>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
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
                                                        " "{area.name.clone()}
                                                    </a>
                                                    " > "
                                                </>
                                            })}
                                            <a href={format!("/categories/{}", cat.id)}>
                                                <span class="breadcrumb-category-code">{cat.id}</span>
                                                " "{cat.name.clone()}
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
                            <div inner_html={content_html()}></div>
                        </div>

                        <footer class="project-footer">
                            {project.jd_category.as_ref().map(|cat| {
                                let projects_signal = use_context::<ReadSignal<Vec<Project>>>()
                                    .expect("Projects context not found!");

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
        <header class="main-header">
            <div class="container header-container">
                <div class="logo">
                    <a href="/" class="logo-link">
                        <span class="logo-icon">JD</span>
                        <span class="logo-text">"Tyler Harpool"</span>
                    </a>
                </div>
                <nav class="main-nav">
                    <a href="/" class="nav-link">"Home"</a>
                    <a href="/areas" class="nav-link">"Areas"</a>
                    <a href="/about" class="nav-link">"About"</a>
                </nav>
            </div>
        </header>
    }
}
// -----------------------------------------
// 7) Areas Page: Display all Areas
// -----------------------------------------
#[component]
fn AreasPage() -> impl IntoView {
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
                                {area.description.clone()}
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

// -----------------------------------------
// 8) Area Detail Page: Display one Area with its Categories
// -----------------------------------------
#[component]
fn AreaDetailPage() -> impl IntoView {
    let areas_signal = use_context::<ReadSignal<Vec<JDArea>>>()
        .expect("Areas context not found!");

    let categories_signal = use_context::<ReadSignal<Vec<JDCategory>>>()
        .expect("Categories context not found!");

    // Get the area ID from URL params
    let params = use_params_map();
    let area_id = move || {
        params.with(|p| {
            p.get("id")
                .and_then(|id| id.parse::<u8>().ok())
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
                                <span class="area-number">{format!("{}-{}", area.id, area.id + 9)}</span>
                                <h1>{area.name.clone()}</h1>
                            </div>

                            <p class="area-description">{area.description.clone()}</p>
                        </header>

                        <h2 class="section-title">"Categories in this Area"</h2>
                        <div class="jd-categories-grid">
                            {move || area_categories().into_iter().map(|category| {
                                view! {
                                    <div class="jd-category-card">
                                        <div class="jd-category-header">
                                            <div class="jd-category-label">
                                                <span class="jd-category-number">{category.id}</span>
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

// -----------------------------------------
// 9) Category Detail Page: Display projects in a category
// -----------------------------------------
#[component]
fn CategoryDetailPage() -> impl IntoView {
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

    // Format date helper function
    let format_date = |date: SystemTime| -> String {
        let secs = date.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
        let days = (secs / 86400) % 30;
        let months = (secs / 2592000) % 12;
        let years = secs / 31104000;
        format!("{}-{:02}-{:02}", years + 1970, months + 1, days + 1)
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

#[component]
fn RenderRelatedProjects(
    project_id: Option<i64>,
    category_id: u8,
    category_link: String,
    view_all_text: String,
) -> impl IntoView {
    let projects_signal = use_context::<ReadSignal<Vec<Project>>>()
        .expect("Projects context not found!");

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
                <div class="no-related-content">
                    <p>"No related articles found in this category."</p>
                </div>
            }
        >
            <div class="related-projects">
                <h3>"Related Articles in this Category"</h3>
                <ul>
                    {move ||
                        projects_signal.get().iter()
                            .filter(|p| p.id != project_id &&
                                  p.jd_category.as_ref().map_or(false, |c| c.id == category_id))
                            .take(3)
                            .map(|p| view! {
                                <li>
                                    <a href={format!("/projects/{}", p.slug)}>
                                        {p.title.clone()}
                                    </a>
                                </li>
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
                <a href={category_link.clone()} class="view-more">
                    {view_all_text.clone()}
                </a>
            </div>
        </Show>
    }
}
