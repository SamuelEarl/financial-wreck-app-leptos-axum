use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(css, "net_worth.module.scss");

#[component]
pub fn NetWorth() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Net Worth"/>

        <h1>"Net Worth"</h1>
    }
}
