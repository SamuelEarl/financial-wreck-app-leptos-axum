// TODO: Can/should I use this module-level attribute instead of all of the other attributes in this module?
// // This attribute ensures this entire module is ignored 
// // if the "docs" feature is missing.
// #![cfg(feature = "docs")]

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
    home_docs::HomeDocs,
    accordions_docs::AccordionsDocs,
    buttons_docs::ButtonsDocs,
    dialogs_docs::DialogsDocs,
    inputs_docs::InputsDocs,
    radio_buttons_docs::RadioButtonsDocs,
    select_docs::SelectDocs,
    tooltips_docs::TooltipsDocs,
};
#[cfg(feature = "docs")]
use crate::nav_links::DOCS_NAV;
#[cfg(feature = "docs")]
use crate::components::layouts::layout::Layout;

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
        <ParentRoute path=path!("/docs") view=move || view! { <Layout nav=&DOCS_NAV /> }>
            <Route path=path!("/home") view=HomeDocs />
            <Route path=path!("/accordions") view=AccordionsDocs />
            <Route path=path!("/buttons") view=ButtonsDocs />
            <Route path=path!("/dialogs") view=DialogsDocs />
            <Route path=path!("/inputs") view=InputsDocs />
            <Route path=path!("/radio-buttons") view=RadioButtonsDocs />
            <Route path=path!("/select") view=SelectDocs />
            <Route path=path!("/tooltips") view=TooltipsDocs />
        </ParentRoute>
    }
    .into_inner()
}
