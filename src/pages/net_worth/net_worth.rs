use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

use crate::utils::format_currency::{format_currency};

import_style!(css, "net_worth.module.scss");

#[component]
pub fn NetWorth() -> impl IntoView {
    provide_meta_context();

    let (net_worth, set_net_worth) = signal(0);
    set_net_worth.set(1_000_000);

    view! {
        <Title text="Financial Wreck | Net Worth"/>

        <h1 class={css::underline}>"Net Worth"</h1>

        <h2>{move || format_currency(net_worth.get(), None)}</h2>

        <p>"Add financial accounts, transfer money between accounts, and add bill pay alerts"</p>

        <h2 class={css::underline}>"Assets"</h2>
        

        <h2 class={css::underline}>"Liabilities"</h2>
    }
}
