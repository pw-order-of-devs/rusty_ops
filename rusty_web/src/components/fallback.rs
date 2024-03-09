use leptos::html::Div;
use leptos::{view, Errors, HtmlElement, RwSignal, SignalWith};

/// Creates a fallback for operation displaying an error message
///
/// # Arguments
///
/// * `errors` - A `RwSignal` of `Errors` to handle any errors.
///
/// # Returns
///
/// An `HtmlElement` of type `Div` that contains the fallback error message.
pub fn fallback(errors: RwSignal<Errors>) -> HtmlElement<Div> {
    errors.with(|errors| {
        errors
            .iter()
            .for_each(|(_, e)| leptos::leptos_dom::log!("{e:?}"));
    });
    // it should parse the errors and display actual message
    view! {
        <div class="error">
            <div>"Failed to execute the operation."</div>
        </div>
    }
}
