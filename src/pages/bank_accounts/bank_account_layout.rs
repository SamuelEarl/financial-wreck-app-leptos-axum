use leptos::prelude::*;
use leptos_router::components::{A, Outlet};
use stylance::*;

import_style!(css, "bank_account_layout.module.scss");

#[component]
pub fn BankAccountLayout() -> impl IntoView {
    view! {
        <div class={css::tabs}>
            <A href="transactions">Transactions</A>
            <A href="budget">Budget</A>
        </div>
        <div>
            <Outlet />
        </div>
    }
}
