use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(style, "transactions.module.scss");

/// Shows progress toward a goal.
#[component]
pub fn Transactions() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Transactions"/>

        <h1 class=style::title>"Transactions"</h1>
    }
}
