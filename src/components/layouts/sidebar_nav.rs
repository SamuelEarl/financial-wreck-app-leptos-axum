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

import_style!(css, "sidebar_nav.module.scss");

// #[derive(Debug)]
// pub struct NavLink {
//     pub text: String,
//     pub url: String,
// }

#[component]
pub fn SidebarNav(
    nav: &'static[NavLink],
    set_nav_is_open: WriteSignal<bool>,
) -> impl IntoView {
    // 1. Get the location signal.
    let location = use_location();

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
            <nav>
                <ul class={css::nav_list}>
                    {
                        nav.iter()
                            .map(|link| view! {
                                <li><A href={link.url}>{link.text}</A></li>
                            })
                            .collect_view()
                    }
                </ul>
                {
                    #[cfg(feature = "docs")]
                    view! {
                        // 2. Reactive Check
                        {
                            move || {
                                // Get the current path string
                                let path = location.pathname.get();

                                // 3. The If Statement
                                if !path.starts_with("/docs") {
                                    Some(view! { 
                                        <ul>
                                            <li><A href="/docs">Docs</A></li>
                                        </ul> 
                                    })
                                } else {
                                    None // Render nothing if we are in /docs
                                }
                            }
                        }
                    }
                }
            </nav>
        </div>
    }
}
