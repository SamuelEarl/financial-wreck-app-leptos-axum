use leptos::prelude::*;
use leptos_router::components::Outlet;
use stylance::*;

use crate::pages::header::Header;

import_style!(css, "layout.module.scss");

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <Header />
        <main>
            <Outlet />
        </main>
    }
}
