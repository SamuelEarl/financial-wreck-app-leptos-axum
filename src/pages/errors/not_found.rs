use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use stylance::*;

import_style!(css, "not_found.module.scss");

#[component]
pub fn NotFound() -> impl IntoView {
    // Only run this code on the server
    // #[cfg(feature = "ssr")]
    // {        
    //     // This context is provided by leptos_axum or leptos_actix
    //     if let Some(response) = use_context::<leptos_axum::ResponseOptions>() {
    //         response.set_status(http::StatusCode::NOT_FOUND);
    //     }
    // }

    provide_meta_context();

    view! {
        <Title text="Financial Wreck | 404 - Not Found"/>

        <h1>"404"</h1>
        <p>"Oops! We couldn't find that page."</p>
        <a href="/dashboard">"Return to your dashboard"</a>
    }
}
