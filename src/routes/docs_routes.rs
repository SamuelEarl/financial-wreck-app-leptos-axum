use leptos::prelude::*;
#[cfg(feature = "docs")]
use leptos_router::{
    components::{ParentRoute, Route},
    // StaticSegment,
    path,
};
#[cfg(not(feature = "docs"))]
use leptos_router::MatchNestedRoutes;

#[cfg(feature = "docs")]
use crate::docs::{
    docs_layout::DocsLayout,
    home::DocsHome,
};

// Import docs only if feature is enabled
#[component(transparent)]
pub fn DocsRoutes() -> impl MatchNestedRoutes + Clone {
    // Return nothing when the docs feature is off in production.
    #[cfg(not(feature = "docs"))]
    return view! { }; // Returns generic AnyView (Empty).

    // Return the docs routes when the docs feature is on in development.
    #[cfg(feature = "docs")]
    view! {
        // Conditional DocsRoutes
        // If the docs feature is OFF, then this block disappears from the binary entirely.
        <ParentRoute path=path!("/docs") view=DocsLayout>
            <Route path=path!("/") view=DocsHome />
        </ParentRoute>
    }
    .into_inner()
}
