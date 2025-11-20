use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(style, "dashboard.module.scss");

/// Shows progress toward a goal.
#[component]
pub fn Dashboard() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Dashboard"/>

        <h1 class=style::title>"Dashboard"</h1>
    }
}
