use leptos::{
    component, create_action, create_local_resource, create_resource, create_signal, view,
    CollectView, ErrorBoundary, IntoView, ReadSignal, SignalGet, SignalGetUntracked, SignalSet,
    SignalWithUntracked, Transition, WriteSignal,
};
use leptos_router::use_params_map;
use std::time::Duration;

use domain::pipelines::RegisterPipeline;

use crate::api::jobs::get_job;
use crate::api::pipelines::{get_pipelines_for_job, run_pipeline};
use crate::components::fallback::fallback;
use crate::utils::dates::parse_date;
use crate::utils::icons::get_pipeline_status_icon;

/// Web page for project details.
#[component]
pub fn JobView() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);
    let id = use_params_map()
        .with_untracked(|params| params.get("id").cloned())
        .unwrap_or_else(String::new);
    let job = create_local_resource(move || id.clone(), get_job);

    let job_view = move || {
        job.and_then(|data| {
            let job_id = data.clone().id;
            let description = data.clone().description.unwrap_or_else(|| "-".to_string());
            let template = base64_url::decode(&data.clone().template).unwrap_or_default();
            let template = String::from_utf8(template).unwrap_or_default();

            let run_pipeline_action = create_action(move |()| {
                let input = job_id.clone();
                async move {
                    if run_pipeline(RegisterPipeline::new(&input)).await.is_ok() {
                        update_counter(counter, set_counter);
                    };
                }
            });

            view! {
                <div class="container container-job-pipelines">
                    <div class="title bold one-line"> "Job: " { data.clone().name } </div>
                    <div class="row-title-button-add">
                        <div class="title"> "Pipelines" </div>
                        <div class="button button-add" on:click=move |_| run_pipeline_action.dispatch(())> "Run >>" </div>
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
                        <JobPipelinesView counter set_counter id=data.clone().id/>
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

fn update_counter(counter: ReadSignal<i32>, set_counter: WriteSignal<i32>) {
    if let Some(mut current) = counter.try_get_untracked() {
        if current == i32::MAX {
            current = -1;
        }
        let _ = set_counter.try_set(current + 1);
    }
}

async fn run_task(counter: ReadSignal<i32>, set_counter: WriteSignal<i32>) {
    loop {
        update_counter(counter, set_counter);
        async_std::task::sleep(Duration::from_secs(10)).await;
    }
}

#[component]
fn JobPipelinesView(
    counter: ReadSignal<i32>,
    set_counter: WriteSignal<i32>,
    #[prop(into)] id: String,
) -> impl IntoView {
    async_std::task::block_on(run_task(counter, set_counter));
    let pipelines = create_resource(
        move || counter.get(),
        move |_| {
            let job_id = id.clone();
            async move { get_pipelines_for_job(job_id).await }
        },
    );

    move || {
        pipelines.and_then(|jobs| {
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
