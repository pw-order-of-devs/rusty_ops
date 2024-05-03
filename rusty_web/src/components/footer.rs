use leptos::{component, view, IntoView};

/// Application footer component
#[component]
#[must_use]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="footer"> { "This is the Footer" } </div>
    }
}
