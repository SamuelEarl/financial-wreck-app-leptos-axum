use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(css, "transactions.module.scss");

#[component]
pub fn Transactions() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Transactions"/>

        <h1>"Transactions"</h1>
    }
}
