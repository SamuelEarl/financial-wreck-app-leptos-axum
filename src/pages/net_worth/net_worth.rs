use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos::logging::log;
use stylance::*;

use crate::components::{
    colors_and_sizes::{BtnVariant, Sizes},
    buttons::button::Button,
};
use crate::utils::format_currency::{format_currency};
use crate::pages::net_worth::net_worth_article::NetWorthArticle;

import_style!(css, "net_worth.module.scss");

#[component]
pub fn NetWorth() -> impl IntoView {
    provide_meta_context();

    let (net_worth, set_net_worth) = signal(0);
    set_net_worth.set(1_000_000);

    view! {
        <Title text="Financial Wreck | Net Worth"/>

        <div class={css::h1_wrapper}>
            <h1 class={css::h1}>"Net Worth"</h1>
            <NetWorthArticle />
        </div>

        <h2>{move || format_currency(net_worth.get(), None)}</h2>

        <br />

        <p>"Add financial accounts, transfer money between accounts, and add bill pay alerts"</p>

        <div class={css::al_wrapper}>
            <h2 class={css::h2}>"Assets"</h2>
            <div class={css::btns_container}>
                <Button
                    variant={BtnVariant::Primary}
                    sizes=Some(Sizes {
                        pv: Some(0),
                        ph: Some(2),
                        ..Default::default()
                    })
                    on:click=move |_| { log!("ADD"); }
                >
                    "Add"
                </Button>
            </div>
        </div>
        
        <br />

        <div class={css::al_wrapper}>
            <h2 class={css::h2}>"Liabilities"</h2>
            <div class={css::btns_container}>
                <Button
                    variant={BtnVariant::Primary}
                    sizes=Some(Sizes {
                        pv: Some(0),
                        ph: Some(2),
                        ..Default::default()
                    })
                    on:click=move |_| { log!("ADD"); }
                >
                    "Add"
                </Button>
            </div>
        </div>

    }
}
