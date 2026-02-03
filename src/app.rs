use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos::logging::log;

use crate::client_router::ClientRouter;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let app_env = env!("APP_ENV");
    log!("app_env: {:?}", app_env);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/financial-wreck-app.css"/>

        // sets the document title
        <Title text="Financial Wreck | Go from Financial Wreck to Carefree Retiree!"/>

        <ClientRouter />
    }
}


#[derive(Clone, Copy)]
pub struct ZStack(pub RwSignal<i32>);

/// This will manage the z-indexes of various components that are stacked on top of each other.
/// Instead of hardcoding z-index: 9999, every time a component (Tooltip, Select, or Dialog) opens, it asks for a "new layer."
/// 1. A global signal starts at base_z = 1000.
/// 2. When Dialog A opens, it calls next(), gets 1001, and assigns it to its style.
/// 3. When Select B opens inside that dialog, it calls next(), gets 1002, and sits on top.
/// 4. When Tooltip C opens, it gets 1003.
impl ZStack {
    pub fn next(&self) -> i32 {
        self.0.update(|z| *z += 1);
        self.0.get_untracked()
    }
}
