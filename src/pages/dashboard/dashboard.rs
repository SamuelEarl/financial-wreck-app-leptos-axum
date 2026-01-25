use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
// use leptos::logging::log;
use stylance::*;

// use crate::components::buttons::button::Button;
// use crate::components::colors_and_sizes::{BtnVariant, Colors, Sizes, ElementWidths};

import_style!(css, "dashboard.module.scss");

#[component]
pub fn Dashboard() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Dashboard"/>

        <h1>"Dashboard"</h1>

        <p>"Users will be able to create their own custom dashboard. Maybe I will provide all of the features here by default and users can rearrange and/or hide the features the way they want."</p>
    }
}
