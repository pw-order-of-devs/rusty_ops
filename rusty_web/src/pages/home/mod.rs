use leptos::{component, view, IntoView};

/// Landing page.
#[component]
#[must_use]
pub fn Home() -> impl IntoView {
    view! {
        <div class="page page-home">
            <div class="container">
                <div class="row">
                    <img src="/static/infinity.svg" alt="" height=100 />
                    <div class="app-name">RustyOps</div>
                </div>
                <div class="row">
                    <a class="app-link button" href="/agents">agents</a>
                    <a class="app-link button" href="/projects">projects</a>
                </div>
            </div>
        </div>
    }
}
