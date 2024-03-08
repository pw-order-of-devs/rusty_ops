use leptos::{component, IntoView, view};

/// Web page for project details.
#[component]
#[must_use]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="page error-page">
            <div class="error-code"> 404 </div>
            <div class="error-message"> "Page not found!" </div>
            <a href="/" class="back-home"> "< go to home page >" </a>
        </div>
    }
}