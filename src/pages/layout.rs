use leptos::prelude::*;
use leptos_router::components::Outlet;
use stylance::*;

import_style!(style, "layout.module.scss");

/// Shows progress toward a goal.
#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <nav>
            "Main Nav"
        </nav>
        <main>
            <Outlet />
        </main>
    }
}
