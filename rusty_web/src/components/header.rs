use leptos::{component, IntoView, view};

#[component]
#[must_use]
pub fn Header() -> impl IntoView {
    view! {
        <div class="header">
            <img src="/static/logo.png" width=50 height=50 quality=100 />
            <a class="title" href="/"> { "RustyOps" } </a>
        </div>
    }
}
