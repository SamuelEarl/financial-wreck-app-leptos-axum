use leptos::prelude::*;
use leptos_router::components::{A, Outlet};
use stylance::*;

import_style!(style, "bank_account_layout.module.scss");

/// Shows progress toward a goal.
#[component]
pub fn BankAccountLayout() -> impl IntoView {
    view! {
        <div class=style::tabs>
            // <A href="/bank-accounts/:uuid/:account_name/:year/:month_name?tab=transactions">Transactions</A>
            <A href="transactions">Transactions</A>
            <A href="budget">Budget</A>
        </div>
        <div>
            <Outlet />
        </div>
    }
}
