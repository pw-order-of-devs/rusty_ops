use leptos::{
    component, create_resource, create_signal, view, CollectView, ErrorBoundary, IntoView,
    SignalGet, SignalSet, Transition,
};

use crate::api::projects::get_projects;
use crate::components::fallback::fallback;
use crate::pages::projects::create::ProjectRegisterModal;

/// Web page for projects list.
#[component]
#[must_use]
pub fn ProjectsList() -> impl IntoView {
    let (modal_visible, set_modal_visible) = create_signal("modal-hidden");
    let projects = create_resource(
        move || modal_visible.get(),
        |_| async move { get_projects().await },
    );

    let projects_view = move || {
        projects.and_then(|data| {
            data.iter()
                .map(|p| {
                    view! {
                        <a href=format!("/projects/{}", p.id) class="card button">
                            <div class="text-field"> { p.clone().name } </div>
                        </a>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <div class="page page-list">
            <div class="row-title-button-add">
                <div class="title-text"> "Your projects:" </div>
                <div class="button button-add" on:click=move |_| set_modal_visible.set("modal-visible")> "Add new" </div>
            </div>
            <div class="container container-projects">
                <Transition fallback=move || { view! { <div>"Loading ..."</div> } }>
                    <ErrorBoundary fallback>
                        { projects_view }
                    </ErrorBoundary>
                </Transition>
            </div>
            <ProjectRegisterModal modal_visible set_modal_visible />
        </div>
    }
}
