use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    // StaticSegment,
    path,
};

use crate::pages::auth::sign_in::SignIn;
use crate::routes::docs_routes::DocsRoutes;
use crate::routes::pages_routes::PagesRoutes;

#[component]
pub fn ClientRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                // <Route path=StaticSegment("") view=HomePage />
                <Route path=path!("/") view=SignIn />
                
                <PagesRoutes />

                // The DocsRoutes will only be included during development
                // and will be left out of the compiled bundle during production builds.
                // See the routes/docs_routes.rs file.
                <DocsRoutes />
            </Routes>
        </Router>
    }
}
