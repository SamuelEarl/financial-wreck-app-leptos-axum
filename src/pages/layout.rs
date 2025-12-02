use leptos::prelude::*;
use leptos_router::components::Outlet;
use stylance::*;

use crate::pages::mobile_header::MobileHeader;
use crate::pages::sidebar_nav::SidebarNav;

import_style!(css, "layout.module.scss");

#[component]
pub fn Layout() -> impl IntoView {
    let (nav_is_open, set_nav_is_open) = signal(false);

    view! {
        <div class={css::layout}>
            <MobileHeader set_nav_is_open=set_nav_is_open />
            <SidebarNav
                nav_is_open=nav_is_open 
                set_nav_is_open=set_nav_is_open
            />
            <main>
                <Outlet />
            </main>
        </div>
    }
}
