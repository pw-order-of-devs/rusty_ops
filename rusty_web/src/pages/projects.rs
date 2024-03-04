use leptos::{CollectView, IntoView, Resource, component, create_local_resource, view, Transition, ErrorBoundary};

use commons::errors::RustyError;
use domain::projects::Project;

use crate::api::projects::{get_projects};

/// Web page for managing projects.
#[component]
pub fn Projects() -> impl IntoView {
    let projects: Resource<_, Result<Vec<Project>, RustyError>> = create_local_resource(|| 0, get_projects);

    // should be a shared component
    let fallback = move |_| {
        view! {
            <div class="error">
                <div>"Failed to fetch the list of projects."</div>
            </div>
        }
    };

    // should be a shared component
    let projects_view = move || {
        projects.and_then(|data| {
            data.iter()
                .map(|p| view! {
                    <div class="card">
                        { p.name.clone() }
                    </div>
                })
                .collect_view()
        })
    };

    view! {
        <div class="page grid-page">
            <div class="title"> { "Your projects:" } </div>
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
