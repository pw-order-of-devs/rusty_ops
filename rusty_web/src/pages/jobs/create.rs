use leptos::{
    component, create_action, create_node_ref, create_signal, html, view, IntoView, NodeRef,
    ReadSignal, SignalGet, SignalSet, WriteSignal,
};

use crate::api::jobs::register_job;
use domain::jobs::RegisterJob;
use domain::templates::pipeline::PipelineTemplate;

/// Modal component for job registration.
#[component]
#[must_use]
pub fn JobRegisterModal(
    /// visibility of the component
    modal_visible: ReadSignal<&'static str>,
    /// visibility of the component - set
    set_modal_visible: WriteSignal<&'static str>,
    /// project id
    project_id: String,
) -> impl IntoView {
    let (form_error, set_form_error) = create_signal(String::new());
    let (job_name_error, set_job_name_error) = create_signal(String::new());
    let (job_template_error, set_job_template_error) = create_signal(String::new());
    let job_name_ref: NodeRef<html::Input> = create_node_ref();
    let job_description_ref: NodeRef<html::Textarea> = create_node_ref();
    let job_template_ref: NodeRef<html::Textarea> = create_node_ref();

    let action_cancel = create_action(move |()| async move {
        action_cancel(
            set_modal_visible,
            set_form_error,
            job_name_ref,
            job_description_ref,
            job_template_ref,
            set_job_name_error,
            set_job_template_error,
        );
    });

    let action_save = create_action(move |input: &RegisterJob| {
        let input = input.to_owned();
        async move {
            action_save(
                input,
                set_modal_visible,
                set_form_error,
                job_name_ref,
                job_description_ref,
                job_template_ref,
                set_job_name_error,
                set_job_template_error,
            )
            .await;
        }
    });

    view! {
        <div class={ move || format!("modal {}", modal_visible.get()) }>
            <div class="title">
                "Add new job"
                <div class="errors"> <span class="error"> { move || form_error.get() } </span> </div>
            </div>
            <div class="content">
                <form class="form-columns" method="GET" action="" id="new-project-form">
                    <div class="form-column">
                        <div class="form-row">
                            <label for="input-job-name"> "Name" </label>
                            <div class="errors"> <span class="error"> { move || job_name_error.get() } </span> </div>
                            <input
                                type="text"
                                id="input-job-name"
                                node_ref=job_name_ref />
                        </div>
                        <div class="form-row">
                            <label for="input-job-description"> "Description" </label>
                            <div class="errors" />
                            <textarea
                                id="input-description-name"
                                node_ref=job_description_ref />
                        </div>
                    </div>
                    <div class="form-column">
                        <div class="form-row">
                            <label for="input-template"> "Pipeline" </label>
                            <div class="errors"> <span class="error"> { move || job_template_error.get() } </span> </div>
                            <textarea
                                class="editing"
                                id="input-template"
                                spellcheck="false"
                                node_ref=job_template_ref />
                        </div>
                    </div>
                </form>
            </div>
            <div class="buttons">
                <div class="button" on:click=move |_| {
                    action_cancel.dispatch(());
                }> "Cancel" </div>
                <div class="button" on:click=move |_| {
                    let name = job_name_ref.get().expect("<input> should be mounted").value();
                    let description = job_description_ref.get().expect("<input> should be mounted").value();
                    let template = job_template_ref.get().expect("<input> should be mounted").value();
                    let template = base64_url::encode(&template);
                    action_save.dispatch(RegisterJob::new(&name, &description, &template, &project_id.clone()));
                }> "Save" </div>
            </div>
        </div>
    }
}

#[allow(clippy::too_many_arguments)]
fn action_cancel(
    set_modal_visible: WriteSignal<&str>,
    set_form_error: WriteSignal<String>,
    job_name_ref: NodeRef<html::Input>,
    job_description_ref: NodeRef<html::Textarea>,
    job_template_ref: NodeRef<html::Textarea>,
    set_job_name_error: WriteSignal<String>,
    set_job_template_error: WriteSignal<String>,
) {
    set_modal_visible.set("modal-hidden");
    set_form_error.set(String::new());
    set_job_name_error.set(String::new());
    set_job_template_error.set(String::new());
    job_name_ref
        .get_untracked()
        .expect("<input> should be mounted")
        .set_value("");
    job_description_ref
        .get_untracked()
        .expect("<input> should be mounted")
        .set_value("");
    job_template_ref
        .get_untracked()
        .expect("<input> should be mounted")
        .set_value("");
}

#[allow(clippy::future_not_send)]
#[allow(clippy::too_many_arguments)]
async fn action_save(
    input: RegisterJob,
    set_modal_visible: WriteSignal<&str>,
    set_form_error: WriteSignal<String>,
    job_name_ref: NodeRef<html::Input>,
    job_description_ref: NodeRef<html::Textarea>,
    job_template_ref: NodeRef<html::Textarea>,
    set_job_name_error: WriteSignal<String>,
    set_job_template_error: WriteSignal<String>,
) {
    let mut errors_name = vec![];
    let mut errors_template = vec![];
    if input.name.is_empty() {
        errors_name.push("• field is required");
    }
    if input.template.is_empty() {
        errors_template.push("• field is required");
    }
    if PipelineTemplate::from_yaml(&input.template).is_err() {
        errors_template.push("• malformed input");
    }

    if errors_name.is_empty() {
        set_job_name_error.set(String::new());
    } else {
        set_job_name_error.set(errors_name.join(" "));
    }

    if errors_template.is_empty() {
        set_job_template_error.set(String::new());
    } else {
        set_job_template_error.set(errors_template.join(" "));
    }

    if [errors_name, errors_template]
        .iter()
        .flatten()
        .next()
        .is_some()
    {
        return;
    }

    match register_job(input).await {
        Ok(_) => {
            set_modal_visible.set("modal-hidden");
            set_form_error.set(String::new());
            job_name_ref
                .get_untracked()
                .expect("<input> should be mounted")
                .set_value("");
            job_description_ref
                .get_untracked()
                .expect("<input> should be mounted")
                .set_value("");
            job_template_ref
                .get_untracked()
                .expect("<input> should be mounted")
                .set_value("");
        }
        Err(_) => {
            set_form_error.set("Error happened while registering a new job".to_string());
        }
    }
}
