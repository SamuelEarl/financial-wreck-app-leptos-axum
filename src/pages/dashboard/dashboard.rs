use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos::logging::log;
use stylance::*;

use crate::components::buttons::button::Button;
use crate::components::colors_and_sizes::{BtnVariant, Colors, Sizes, ElementWidths};

import_style!(css, "dashboard.module.scss");

#[component]
pub fn Dashboard() -> impl IntoView {
    provide_meta_context();

    // let btn_colors = Colors {
    //     bg: String::from("#000000"),
    //     fg: String::from("#ffffff"),
    //     br: String::from("red"),
    //     ol: String::from("blue"),
    // };

    view! {
        <Title text="Financial Wreck | Dashboard"/>

        <h1 class={css::title}>"Dashboard"</h1>

        <div class={css::btn_wrapper}>
            <Button
                sizes=Some(Sizes {
                    fs: Some(5),
                    fw: Some("bold".to_string()),
                    pv: Some(5),
                    ph: Some(5),
                })
                colors=Some(Colors {
                    bg: "var(--dark-red)".to_string(),
                    fg: "var(--white)".to_string(),
                    br: "var(--black)".to_string(),
                    ol: "var(--black)".to_string(),
                })
                on:click=move |_| { log!("CLICKED"); }
                on:mouseenter=move |_| { log!("HOVERED"); }
            >
                "Click"
            </Button>

            <Button
                variant={BtnVariant::Secondary}
                inverted={false}
                width=ElementWidths::Full
            >
                "Click"
            </Button>
        </div>
    }
}
