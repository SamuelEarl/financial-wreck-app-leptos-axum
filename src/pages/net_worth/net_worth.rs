use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos::logging::log;
use stylance::*;

use crate::components::{
    accordions::accordion::AccordionItem,
    buttons::button::Button,
    colors_and_sizes::BtnVariant,
};
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

        <AccordionItem id="net_worth" title="What is net worth?">
            <p>"Your net worth is the difference between your assets and your liabilities. To calculate your net worth, add all your assets and liabilities to this page."</p>
            <p>
                <Button
                    variant={BtnVariant::Primary}
                    on:click=move |_| { log!("CLICKED"); }
                >
                    "Learn More"
                </Button>
            </p>
        </AccordionItem>

        <br />

        <p>"Add financial accounts, transfer money between accounts, and add bill pay alerts"</p>

        <h2 class={css::underline}>"Assets"</h2>
        
        <br />

        <h2 class={css::underline}>"Liabilities"</h2>
    }
}
