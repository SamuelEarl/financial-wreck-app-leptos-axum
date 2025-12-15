use leptos::prelude::*;
// use leptos::logging::log;
use leptos_router::components::A;
use stylance::*;

use crate::components::colors_and_sizes::{Colors, Sizes};
use crate::components::{buttons::button::Button, icons::icon::Icon};

import_style!(css, "mobile_header.module.scss");

#[component]
pub fn MobileHeader(#[prop(into)] set_nav_is_open: WriteSignal<bool>) -> impl IntoView {
    view! {
        <header class={css::mobile_header}>
            <div class={css::header_content}>
                <div class="open_menu_btn_container">
                    <Button
                        sizes=Some(Sizes {
                            pv: Some(0),
                            ph: Some(0),
                            ..Default::default()
                        })
                        colors=Some(Colors {
                            bg: "transparent".to_string(),
                            fg: "var(--white)".to_string(),
                            br: "transparent".to_string(),
                            ol: "transparent".to_string(),
                        })
                        // Open menu
                        on:click=move |_| set_nav_is_open.set(true)
                    >
                        <Icon
                            icon="material-symbols:menu".to_string()
                            attr:style="font-size: var(--size-8)"
                        />
                    </Button>
                </div>
                <div class={css::logo_wrapper}>
                    <A href="/dashboard">
                        <img class={css::logo} src="/images/logo-white.svg" alt="logo" />
                    </A>
                </div>
                <div>
                    <Icon
                        icon="material-symbols:menu".to_string()
                        attr:style="font-size: var(--size-8); color: transparent;"
                    />
                </div>
            </div>
        </header>
    }
}
