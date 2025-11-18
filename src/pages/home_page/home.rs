use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(style, "home.module.scss");

/// Shows progress toward a goal.
#[component]
pub fn Home() -> impl IntoView {
    provide_meta_context();

    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <Title text="Financial Wreck | Home"/>

        <h1 class=style::title>"Welcome to the Home Page!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <br/><br/>
        <img src="/images/android-chrome-512x512.png" alt="Logo" />
    }
}
