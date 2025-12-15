use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;
use leptos_use::use_window;
use leptos::logging::log;
use stylance::*;

use crate::nav_links::NavLink;

use crate::components::layouts::{
    mobile_header::MobileHeader,
    sidebar_nav::SidebarNav,
};

import_style!(css, "layout.module.scss");

#[component]
pub fn Layout(nav: &'static[NavLink]) -> impl IntoView {
    let (nav_is_open, set_nav_is_open) = signal(false);

    // 1. Get the location signal.
    let location = use_location();

    // 2. Create an Effect that runs whenever the path changes.
    Effect::new(move |_| {
        // Accessing .get() subscribes this effect to changes
        let current_path = location.pathname.get();

        // --- LOGIC STARTS HERE ---
        // This runs immediately on load AND after every navigation.
        
        // Example: Log to console (or send to Google Analytics).
        log!("Navigated to: {}", current_path);

         // Returns `UseWindow` which wraps `Option<Window>`
        let window = use_window();

        // Scroll to top of window
        if let Some(win) = window.as_ref() {
            win.scroll_to_with_x_and_y(0.0, 0.0);
        }

        // Close the sidebar_nav after navigation. (This only matters for mobile screens.)
        set_nav_is_open.set(false);
    });

    view! {
        <div class={css::layout}>
            <MobileHeader set_nav_is_open=set_nav_is_open />
            <nav 
                class={css::layout_nav}
                style:right=move || if nav_is_open.get() { "0" } else { "110vw" }
            >
                <SidebarNav
                    nav=nav
                    set_nav_is_open=set_nav_is_open
                />
            </nav>
            <main>
                <Outlet />
            </main>
        </div>
    }
}
