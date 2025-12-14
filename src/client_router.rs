use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    // StaticSegment,
    path,
};

use crate::pages::auth::sign_in::SignIn;
// Import the DocsRoutes only if the docs feature is enabled.
#[cfg(feature = "docs")]
use crate::routes::docs_routes::DocsRoutes;
use crate::routes::pages_routes::PagesRoutes;

// use crate::pages::errors::not_found::NotFound;

#[component]
pub fn ClientRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                // <Route path=StaticSegment("") view=HomePage />
                <Route path=path!("/") view=SignIn />
                
                <PagesRoutes />

                // TODO: Figure out how to conditionally compile DocsRoutes so it is excluded in a production build.
                // #[cfg(feature = "docs")]
                <DocsRoutes />
            </Routes>
        </Router>
    }
}
