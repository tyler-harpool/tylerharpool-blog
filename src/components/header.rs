use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="main-header">
            <div class="container header-container">
                <div class="logo">
                    <a href="/" class="logo-link">
                        <span class="logo-icon">TH</span>
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
