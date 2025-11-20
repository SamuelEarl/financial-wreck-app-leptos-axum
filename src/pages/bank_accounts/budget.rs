use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(style, "budget.module.scss");

/// Shows progress toward a goal.
#[component]
pub fn Budget() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Budget"/>

        <h1 class=style::title>"Budget"</h1>
    }
}
