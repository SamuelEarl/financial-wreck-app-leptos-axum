use leptos::prelude::*;
use leptos_router::{
    components::{Route, ParentRoute},
    // StaticSegment,
    MatchNestedRoutes,
    path,
};

use crate::docs::docs_layout::DocsLayout;
use crate::docs::home::DocsHome;

// Import docs only if feature is enabled
#[cfg(feature = "docs")]
#[component(transparent)]
pub fn DocsRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        // Conditional Docs Route
        // If feature is OFF, this block disappears from the binary entirely.
        <ParentRoute path=path!("/docs") view=DocsLayout>
            <Route path=path!("/") view=DocsHome />
        </ParentRoute>
    }
    .into_inner()
}
