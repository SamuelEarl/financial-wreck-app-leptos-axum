use leptos::prelude::*;
use leptos::attr::any_attribute::AnyAttribute;
use stylance::*;

import_style!(css, "input.module.scss");

#[component]
pub fn Input(
    /// This attribute allows you to pass any standard HTML attributes
    /// such as type, placeholder, disabled, etc.
    #[prop(attrs)] attributes: Vec<AnyAttribute>,
) -> impl IntoView {
    view! {
        <input
            class={css::input}
            // This spreads all the passed attributes onto the underlying element
            {..attributes}
        />
    }
}
