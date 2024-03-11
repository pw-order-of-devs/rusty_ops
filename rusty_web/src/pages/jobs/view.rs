use leptos::{
    component, create_local_resource, view, CollectView, ErrorBoundary, IntoView,
    SignalWithUntracked, Transition,
};
use leptos_router::use_params_map;

use crate::api::jobs::get_job;
use crate::api::pipelines::get_pipelines_for_job;
use crate::components::fallback::fallback;
use crate::utils::dates::parse_date;
use crate::utils::icons::get_pipeline_status_icon;

/// Web page for project details.
#[component]
pub fn JobView() -> impl IntoView {
    let id = use_params_map()
        .with_untracked(|params| params.get("id").cloned())
        .unwrap_or_else(String::new);
    let job = create_local_resource(move || id.clone(), get_job);

    let job_view = move || {
        job.and_then(|data| {
            let description = data.clone().description.unwrap_or_else(|| "-".to_string());
            let template = base64_url::decode(&data.clone().template).unwrap_or_default();
            let template = String::from_utf8(template).unwrap_or_default();
            leptos::leptos_dom::log!("{template}");

            view! {
                <div class="container container-job-pipelines">
                    <div class="title bold one-line"> "Job: " { data.clone().name } </div>
                    <div class="row-title-button-add">
                        <div class="title"> "Pipelines" </div>
                        <div class="button button-add"> "Run >>" </div>
                    </div>
                </div>
                <div class="container container-job-pipelines-content">
                    <div class="job-details">
                        <label> "description:" </label>
                        <div> { description } </div>
                    </div>
                    <div class="job-template">
                        <textarea disabled> { template } </textarea>
                    </div>
                    <div class="list-job-pipelines scrollable">
                        <JobPipelinesView id=data.clone().id/>
                    </div>
                </div>
            }
        })
        .collect_view()
    };

    view! {
        <div class="page page-view">
            <Transition fallback=move || { view! { <div>"Loading ..."</div> } }>
                <ErrorBoundary fallback>
                    { job_view }
                </ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
fn JobPipelinesView(#[prop(into)] id: String) -> impl IntoView {
    let jobs = create_local_resource(move || id.clone(), get_pipelines_for_job);

    move || {
        jobs.and_then(|jobs| {
            jobs.iter()
                .map(|data| {
                    let status_icon = get_pipeline_status_icon(&data.status);
                    let date = parse_date(&data.start_date);

                    view! {
                        <a href=format!("/pipelines/{}", data.clone().id) class="card button">
                            <div class="row">
                                <img src=format!("/static/{}.svg", status_icon) width=16 height=16/>
                                <div> "#" { data.clone().number } " @ " { date } </div>
                            </div>
                        </a>
                    }
                })
                .collect_view()
        })
    }
}
