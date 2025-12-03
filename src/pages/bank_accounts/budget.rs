use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(css, "budget.module.scss");

#[component]
pub fn Budget() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Budget"/>

        <h1>"Budget"</h1>
    }
}
