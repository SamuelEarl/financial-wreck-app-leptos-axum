use leptos::prelude::*;
use leptos_router::components::A;
use leptos::logging::log;
use stylance::*;

use crate::pages::header::NavLink;
use crate::components::{
    buttons::button::Button,
    icons::icon::Icon,
};
use crate::components::colors_and_sizes::{Colors, Sizes};

import_style!(css, "header_mobile.module.scss");

#[component]
pub fn HeaderMobile(nav: [NavLink; 4]) -> impl IntoView {
    view! {
        <header class={css::mobile_header}>
            <div class={css::content}>
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
                        on:click=move |_| { log!("CLICKED"); }
                    >
                        <Icon
                            icon="material-symbols:menu".to_string()
                            attr:style="font-size: var(--size-8)"
                        />
                    </Button>
                </div>
                <div class={css::logo_wrapper}>
                    <A href="/">
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
            <ul>
                {
                    nav.into_iter()
                        .map(|link| view! {
                            <li><A href={link.url}>{link.label}</A></li>
                        })
                        .collect_view()
                }
            </ul>
        </header>
    }
}
