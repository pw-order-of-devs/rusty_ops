use leptos::{
    component, create_resource, create_signal, view, CollectView, ErrorBoundary, IntoView,
    ReadSignal, SignalGet, SignalGetUntracked, SignalSet, Transition, WriteSignal,
};
use std::time::Duration;

use crate::api::agents::get_agents;
use crate::components::fallback::fallback;

#[component]
#[must_use]
pub fn AgentsList() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);
    async_std::task::block_on(run_task(counter, set_counter));
    let agents = create_resource(move || counter.get(), |_| async move { get_agents().await });

    let agents_view = move || {
        agents.and_then(|data| {
            if data.total == 0 {
                (0..1)
                    .map(|_| view! { <div class="list-empty">No registered agents</div> })
                    .collect_view()
            } else {
                data.entries
                    .iter()
                    .map(|p| {
                        let expiry = chrono::DateTime::from_timestamp(p.clone().expiry, 0)
                            .unwrap()
                            .to_rfc2822();
                        view! {
                            <div class="card">
                                <div class="text-field"> "Agent id: " { p.clone().id } </div>
                                <div class="text-field"> "Expires at: " { expiry } </div>
                            </div>
                        }
                    })
                    .collect_view()
            }
        })
    };

    view! {
        <div class="page page-list">
            <div class="row-title-button-add">
                <div class="title-text"> "Available agents:" </div>
            </div>
            <div class="container container-agents-list scrollable">
                <Transition fallback=move || { view! { <div class="pd-2-rem">"Loading ..."</div> } }>
                    <ErrorBoundary fallback>
                        { agents_view }
                    </ErrorBoundary>
                </Transition>
            </div>
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
