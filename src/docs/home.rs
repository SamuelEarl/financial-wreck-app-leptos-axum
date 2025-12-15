use leptos::prelude::*;
use leptos_router::components::Outlet;

// This attribute ensures this entire module is ignored 
// if the "docs" feature is missing.
#[cfg(feature = "docs")] 
#[component]
pub fn DocsHome() -> impl IntoView {
    view! {
        <div class="docs-container">
            <h1>"Component Library"</h1>
            <p>"Welcome..."</p>
        </div>
    }
}
