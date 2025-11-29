use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

use crate::components::buttons::button::Button;

import_style!(css, "dashboard.module.scss");

#[component]
pub fn Dashboard() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Dashboard"/>

        <h1 class=css::title>"Dashboard"</h1>

        <Button>Click</Button>
    }
}
