use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(css, "goals.module.scss");

#[component]
pub fn Goals() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Goals"/>

        <h1>"Goals"</h1>
    }
}
