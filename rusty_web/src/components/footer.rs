use leptos::{component, view, IntoView};

#[component]
#[must_use]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="footer"> { "This is the Footer" } </div>
    }
}
