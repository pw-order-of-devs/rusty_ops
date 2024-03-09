use leptos::{
    component, create_local_resource, view, CollectView, ErrorBoundary, IntoView, Transition,
};

use crate::api::projects::get_projects;
use crate::components::fallback::fallback;

/// Web page for projects list.
#[component]
pub fn ProjectsList() -> impl IntoView {
    let projects = create_local_resource(|| 0, get_projects);

    let projects_view = move || {
        projects.and_then(|data| {
            data.iter()
                .map(|p| {
                    view! {
                        <a href=format!("/projects/{}", p.id) class="card button">
                            { p.clone().name }
                        </a>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <div class="page list-page">
            <div class="title"> "Your projects:" </div>
            <div class="container">
                <Transition fallback=move || { view! { <div>"Loading ..."</div> } }>
                    <ErrorBoundary fallback>
                        { projects_view }
                    </ErrorBoundary>
                </Transition>
            </div>
        </div>
    }
}
