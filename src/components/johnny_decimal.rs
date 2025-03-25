use leptos::prelude::*;

/// A reusable component for displaying Johnny Decimal notation
/// with consistent styling across the application
///
///

#[component]
pub fn JohnnyDecimal(
    #[prop(into)] area_id: String,
    #[prop(into)] category_id: String,
    #[prop(into)] category_name: String,
    #[prop(optional, default = true)] show_name: bool,
) -> impl IntoView {
    view! {
        <div class="jd-container">
            <div class="jd-notation">
                <span class="jd-area">{area_id}</span>
                <span class="jd-separator">.</span>  // Changed to standard period
                <span class="jd-category">{category_id}</span>
            </div>
            {show_name.then(|| {
                view! {
                    <div class="jd-name">{category_name}</div>
                }
            })}
        </div>
    }
}

/// A component for displaying Johnny Decimal area ranges
/// Used in area listing pages
#[component]
pub fn JohnnyDecimalRange(
    #[prop(into)] start: String,
    #[prop(into)] end: String,
    #[prop(into)] name: String,
) -> impl IntoView {
    view! {
        <div class="jd-range-container">
            <div class="jd-range">{start}-{end}</div>
            <div class="jd-range-name">{name}</div>
        </div>
    }
}

/// A component for article listings with Johnny Decimal notation
#[component]
pub fn JohnnyDecimalHeader(
    #[prop(into)] area_id: String,
    #[prop(into)] category_id: String,
    #[prop(into)] title: String,
) -> impl IntoView {
    view! {
        <div class="jd-header">
            <div class="jd-header-left">
                <div class="jd-header-notation">
                    <span class="jd-header-area">{area_id}</span>
                    <span class="jd-header-separator">.</span>
                    <span class="jd-header-category">{category_id}</span>
                </div>
            </div>
            <div class="jd-header-title">
                <span>{title}</span>
            </div>
        </div>
    }
}

/// A component for displaying a category badge (the circle with number)
#[component]
pub fn CategoryBadge(
    #[prop(into)] id: String,
) -> impl IntoView {
    view! {
        <span class="jd-category-badge">{id}</span>
    }
}

/// A component for breadcrumb navigation with Johnny Decimal elements
#[component]
pub fn JohnnyDecimalBreadcrumbs(
    #[prop(into)] area_id: String,
    #[prop(into)] area_name: String,
    #[prop(into)] category_id: String,
    #[prop(into)] category_name: String,
) -> impl IntoView {
    view! {
        <div class="jd-breadcrumbs">
            <a href="/areas">
                <span>Areas</span>
            </a>
            <span class="jd-breadcrumb-separator">/</span>
            <a href={format!("/areas/{}", area_id.clone())}>
                <span class="jd-breadcrumb-area">{area_id.clone()}</span>
                <span>{area_name}</span>
            </a>
            <span class="jd-breadcrumb-separator">/</span>
            <a href={format!("/categories/{}", category_id.clone())}>
                <span class="jd-breadcrumb-category">{category_id.clone()}</span>
                <span>{category_name}</span>  // Added this back
            </a>
        </div>
    }
}
