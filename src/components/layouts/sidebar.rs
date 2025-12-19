use leptos::prelude::*;
// use leptos::logging::log;
use leptos_router::{
    components::A,
    hooks::use_location,
};
use stylance::*;

use crate::nav_links::NavLink;

use crate::components::colors_and_sizes::{Colors, Sizes};
use crate::components::{buttons::button::Button, icons::icon::Icon};

import_style!(css, "sidebar.module.scss");

#[component]
pub fn Sidebar(
    nav: &'static[NavLink],
    set_nav_is_open: WriteSignal<bool>,
) -> impl IntoView {
    // Get the location signal.
    let location = use_location();
    // Create a derived signal for readability
    let not_docs = move || !location.pathname.get().starts_with("/docs");

    view! {
        <div class={css::nav_container}>
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
            <nav class={css::sidebar_nav}>
                <ul class={css::nav_list}>
                    {
                        nav.iter()
                            .map(|link| view! {
                                <li class={css::list_item}><A href={link.url}>{link.text}</A></li>
                            })
                            .collect_view()
                    }
                </ul>
                {
                    #[cfg(feature = "docs")]
                    view! {
                        <Show
                            // If this is TRUE, show children
                            when=not_docs 
                            // If FALSE, show nothing (or put a specific fallback view here)
                            fallback=|| view! {} 
                        >
                            <ul class={css::dev_list}>
                                <li class={css::list_item}><A href="/docs">Docs</A></li>
                            </ul>
                        </Show>
                    }
                }
            </nav>
        </div>
    }
}
