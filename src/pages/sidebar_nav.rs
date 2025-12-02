use leptos::prelude::*;
// use leptos::logging::log;
use leptos_router::components::A;
use stylance::*;

use crate::components::colors_and_sizes::{Colors, Sizes};
use crate::components::{buttons::button::Button, icons::icon::Icon};

import_style!(css, "sidebar_nav.module.scss");

#[derive(Debug, Clone)]
pub struct NavLink {
    pub text: String,
    pub url: String,
}

#[component]
pub fn SidebarNav(
    #[prop(into)] nav_is_open: Signal<bool>,
    set_nav_is_open: WriteSignal<bool>,
) -> impl IntoView {
    let main_nav: [NavLink; 4] = [
        NavLink { text: String::from("Dashboard"), url: String::from("/dashboard"), },
        NavLink { text: String::from("Net Worth"), url: String::from("/net-worth"), },
        NavLink { text: String::from("Transactions"), url: String::from("/transactions"), },
        NavLink { text: String::from("Budgets"), url: String::from("/budgets"), },
    ];

    view! {
        <div
            class={css::nav_container}
            style:right=move || if nav_is_open.get() { "0" } else { "110vw" }
        >
            <div class={css::nav_container_header}>
                <div class={css::close_menu_btn_container}>
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
                        // Close menu
                        on:click=move |_| set_nav_is_open.set(false)
                    >
                        <Icon
                            icon="material-symbols:close".to_string()
                            attr:style="font-size: var(--size-8)"
                        />
                    </Button>
                </div>
                <div class={css::logo_wrapper}>
                    <A href="/dashboard">
                        <img class={css::logo} src="/images/logo-white.svg" alt="logo" />
                    </A>
                </div>
                <div class={css::spacer}>
                    <Icon
                        icon="material-symbols:close".to_string()
                        attr:style="font-size: var(--size-8); color: transparent;"
                    />
                </div>
            </div>
            <nav>
                <ul>
                    {
                        main_nav.into_iter()
                            .map(|link| view! {
                                <li><A href={link.url}>{link.text}</A></li>
                            })
                            .collect_view()
                    }
                </ul>
            </nav>
        </div>
    }
}
