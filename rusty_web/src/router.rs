use leptos::{component, IntoView, view};
use leptos_router::{Route, Router, Routes};

use crate::pages::projects::list::ProjectsList;
use crate::pages::projects::view::ProjectView;

#[component]
pub fn RustyRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/projects" view=ProjectsList/>
                <Route path="/projects/:id" view=ProjectView/>
                <Route path="/*any" view=|| view! { <h1 style="margin: 100px; color: white;">Page not found ..</h1> }/>
            </Routes>
        </Router>
    }
}