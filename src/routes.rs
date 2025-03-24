use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, path,
};

use crate::pages::{
    HomePage, AboutPage, AreasPage, AreaDetailPage, CategoryDetailPage, ProjectPage
};

/// Sets up the application's routes
pub fn setup_routes() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route
                        path=StaticSegment("")
                        view=move || {
                            view! {
                                <Suspense fallback=move || view! { <p>"Loading projects..."</p> }>
                                    <HomePage />
                                </Suspense>
                            }
                        }
                    />
                    <Route path=StaticSegment("/about") view=AboutPage/>
                    <Route path=path!("/projects/:slug") view=ProjectPage/>
                    <Route path=path!("/areas") view=AreasPage/>
                    <Route path=path!("/areas/:id") view=AreaDetailPage/>
                    <Route path=path!("/categories/:id") view=CategoryDetailPage/>
                </Routes>
            </main>
        </Router>
    }
}