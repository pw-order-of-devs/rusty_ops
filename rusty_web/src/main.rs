//! `rusty_web` - web ui for `rusty_ops`

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::similar_names)]
#![cfg_attr(test, deny(rust_2018_idioms))]

use leptos::{component, mount_to_body, view, IntoView};
use leptos_meta::provide_meta_context;

use crate::components::header::Header;
use crate::router::RustyRouter;

/// Server API module.
pub mod api;

/// Module containing ui components.
pub mod components;

/// Module containing pages.
pub mod pages;

/// Module containing routes definitions.
pub mod router;

/// Module containing utility functions.
pub mod utils;

fn main() {
    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Header/>
        <RustyRouter/>
    }
}
