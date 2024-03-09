use crate::pages::errors::not_found::NotFoundPage;
use crate::pages::home::Home;
use crate::pages::jobs::view::JobView;
use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes};

use crate::pages::projects::list::ProjectsList;
use crate::pages::projects::view::ProjectView;

#[component]
#[must_use]
pub fn RustyRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/projects" view=ProjectsList/>
                <Route path="/projects/:id" view=ProjectView/>
                <Route path="/jobs/:id" view=JobView/>
                <Route path="/*any" view=NotFoundPage/>
            </Routes>
        </Router>
    }
}
