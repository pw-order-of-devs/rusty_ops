use leptos::{CollectView, component, create_local_resource, ErrorBoundary, IntoView, SignalWithUntracked, Transition, view};
use leptos_router::use_params_map;

use crate::api::projects::get_project;
use crate::components::fallback::fallback;

/// Web page for project details.
#[component]
pub fn ProjectView() -> impl IntoView {
    let id = use_params_map()
        .with_untracked(|params| params.get("id").cloned())
        .unwrap_or_else(String::new);
    let project = create_local_resource(move || id.clone(), get_project);

    let project_view = move || {
        project.and_then(|data| {
            view! {
                <div class="container">
                    <div class="title"> { data.clone().name } </div>
                    <div class="title"> "Jobs:" </div>
                </div>
                <div class="container">
                    <div class="project-details">
                        "other metadata about the project"
                    </div>
                    <div class="project-jobs"> {
                        data.clone().jobs.unwrap_or_default().iter()
                            .map(|job| view! {
                                <div class="card">
                                    <div> { job.clone().name } </div>
                                    <br />
                                    <div class="text-field"> { job.clone().description } </div>
                                </div>
                            }).collect_view()
                    } </div>
                </div>
            }
        }).collect_view()
    };

    view! {
        <div class="page view-page">
            <Transition fallback=move || { view! { <div>"Loading ..."</div> } }>
                <ErrorBoundary fallback>
                    { project_view }
                </ErrorBoundary>
            </Transition>
        </div>
    }
}
