use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(css, "budget.module.scss");

#[component]
pub fn Budgets() -> impl IntoView {
    provide_meta_context();

    // /bank-accounts/:uuid/:account_name/:year/:month_name/budgets
    // /bank-accounts/:uuid/:account_name/:year/:month_name/transactions

    view! {
        <Title text="Financial Wreck | Budgets"/>

        <h1 class={css::title}>"Budgets"</h1>

        <p>Pick a bank account to view the associated budget:</p>
    }
}
