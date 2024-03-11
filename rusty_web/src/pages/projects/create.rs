use leptos::{
    component, create_action, create_node_ref, create_signal, html, view, IntoView, NodeRef,
    ReadSignal, SignalGet, SignalSet, WriteSignal,
};

use domain::projects::RegisterProject;

use crate::api::projects::register_project;

/// Modal component for project registration.
#[component]
#[must_use]
pub fn ProjectRegisterModal(
    /// visibility of the component
    modal_visible: ReadSignal<&'static str>,
    /// visibility of the component - set
    set_modal_visible: WriteSignal<&'static str>,
) -> impl IntoView {
    let (form_error, set_form_error) = create_signal(String::new());
    let (project_name_error, set_project_name_error) = create_signal(String::new());
    let (project_repository_error, set_project_repository_error) = create_signal(String::new());
    let project_name_ref: NodeRef<html::Input> = create_node_ref();
    let project_repository_ref: NodeRef<html::Input> = create_node_ref();

    let action_cancel = create_action(move |()| async move {
        action_cancel(
            set_modal_visible,
            set_form_error,
            project_name_ref,
            project_repository_ref,
            set_project_name_error,
            set_project_repository_error,
        );
    });

    let action_save = create_action(move |input: &RegisterProject| {
        let input = input.to_owned();
        async move {
            action_save(
                input,
                set_modal_visible,
                set_form_error,
                project_name_ref,
                project_repository_ref,
                set_project_name_error,
                set_project_repository_error,
            )
            .await;
        }
    });

    view! {
        <div class={ move || format!("modal {}", modal_visible.get()) }>
            <div class="title">
                "Add new project"
                <div class="errors"> <span class="error"> { move || form_error.get() } </span> </div>
            </div>
            <div class="content">
                <form method="GET" action="" id="new-project-form">
                    <div class="form-row">
                        <label for="input-project-name"> "Name" </label>
                        <div class="errors"> <span class="error"> { move || project_name_error.get() } </span> </div>
                        <input
                            type="text"
                            id="input-project-name"
                            node_ref=project_name_ref />
                    </div>
                    <div class="form-row">
                        <label for="input-project-url"> "Repository URL" </label>
                        <div class="errors"> <span class="error"> { move || project_repository_error.get() } </span> </div>
                        <input
                            type="text"
                            id="input-project-url"
                            node_ref=project_repository_ref />
                    </div>
                </form>
            </div>
            <div class="buttons">
                <div class="button" on:click=move |_| {
                    action_cancel.dispatch(());
                }> "Cancel" </div>
                <div class="button" on:click=move |_| {
                    let name = project_name_ref.get().expect("<input> should be mounted").value();
                    let url = project_repository_ref.get().expect("<input> should be mounted").value();
                    action_save.dispatch(RegisterProject::new(&name, &url));
                }> "Save" </div>
            </div>
        </div>
    }
}

#[allow(clippy::too_many_arguments)]
fn action_cancel(
    set_modal_visible: WriteSignal<&str>,
    set_form_error: WriteSignal<String>,
    project_name_ref: NodeRef<html::Input>,
    project_repository_ref: NodeRef<html::Input>,
    set_project_name_error: WriteSignal<String>,
    set_project_repository_error: WriteSignal<String>,
) {
    set_modal_visible.set("modal-hidden");
    set_form_error.set(String::new());
    set_project_name_error.set(String::new());
    set_project_repository_error.set(String::new());
    project_name_ref
        .get_untracked()
        .expect("<input> should be mounted")
        .set_value("");
    project_repository_ref
        .get_untracked()
        .expect("<input> should be mounted")
        .set_value("");
}

#[allow(clippy::future_not_send)]
#[allow(clippy::too_many_arguments)]
async fn action_save(
    input: RegisterProject,
    set_modal_visible: WriteSignal<&str>,
    set_form_error: WriteSignal<String>,
    project_name_ref: NodeRef<html::Input>,
    project_repository_ref: NodeRef<html::Input>,
    set_project_name_error: WriteSignal<String>,
    set_project_repository_error: WriteSignal<String>,
) {
    let mut errors_name = vec![];
    let mut errors_url = vec![];
    if input.name.is_empty() {
        errors_name.push("• field is required");
    }
    if input.url.is_empty() {
        errors_url.push("• field is required");
    }
    if url::Url::parse(&input.url).is_err() {
        errors_url.push("• invalid url");
    }

    if errors_name.is_empty() {
        set_project_name_error.set(String::new());
    } else {
        set_project_name_error.set(errors_name.join(" "));
    }

    if errors_url.is_empty() {
        set_project_repository_error.set(String::new());
    } else {
        set_project_repository_error.set(errors_url.join(" "));
    }

    if [errors_name, errors_url].iter().flatten().next().is_some() {
        return;
    }

    match register_project(input).await {
        Ok(_) => {
            set_modal_visible.set("modal-hidden");
            set_form_error.set(String::new());
            project_name_ref
                .get_untracked()
                .expect("<input> should be mounted")
                .set_value("");
            project_repository_ref
                .get_untracked()
                .expect("<input> should be mounted")
                .set_value("");
        }
        Err(_) => {
            set_form_error.set("Error happened while registering a new project".to_string());
        }
    }
}
