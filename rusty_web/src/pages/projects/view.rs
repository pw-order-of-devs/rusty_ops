use leptos::{CollectView, component, create_local_resource, ErrorBoundary, IntoView, SignalWithUntracked, Transition, view};
use leptos_router::use_params_map;

use crate::api::jobs::get_jobs_for_project;
use crate::api::pipelines::get_last_pipeline_for_job;
use crate::api::projects::get_project;
use crate::components::fallback::fallback;
use crate::utils::dates::parse_date;
use crate::utils::icons::get_pipeline_status_icon;

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
                    <div class="project-jobs">
                        <ProjectJobsView id=data.clone().id/>
                    </div>
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

#[component]
fn ProjectJobsView(#[prop(into)] id: String) -> impl IntoView {
    let jobs = create_local_resource(move || id.clone(), get_jobs_for_project);

    move || { jobs.and_then(|jobs| {
        jobs.iter().map(|data| view! {
            <a href=format!("/jobs/{}", data.clone().id) class="card button">
                <div class="row">
                    <div> { data.clone().name }":" </div>
                    <div> { data.clone().description } </div>
                </div>
                <ProjectJobLastPipelineView id=data.clone().id/>
            </a>
        }).collect_view()
    }) }
}

#[component]
fn ProjectJobLastPipelineView(#[prop(into)] id: String) -> impl IntoView {
    let last_pipe = create_local_resource(move || id.clone(), get_last_pipeline_for_job);

    move || { last_pipe.and_then(|pipe| {
        pipe.as_ref().map_or_else(|| view! {
            <div class="row" />
        }, |data| {
            let status_icon = get_pipeline_status_icon(&data.status);
            let date = parse_date(&data.start_date);

            view! {
                <div class="row">
                    <img src=format!("/static/{}.svg", status_icon) width=16 height=16/>
                    <div> "#" { data.clone().number } " @ " { date } </div>
                </div>
            }
        })
    }).collect_view() }
}
