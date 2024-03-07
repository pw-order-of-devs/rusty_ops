use leptos::{component, IntoView, view};

/// Landing page.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="page home-page">
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
