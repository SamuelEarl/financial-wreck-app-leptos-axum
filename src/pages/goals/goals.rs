use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(style, "goals.module.scss");

/// Shows progress toward a goal.
#[component]
pub fn Goals() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Goals"/>

        <h1 class=style::title>"Goals"</h1>
    }
}
