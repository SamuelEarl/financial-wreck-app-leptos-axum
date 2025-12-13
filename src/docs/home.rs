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
            <nav>
                <a href="/component-library/buttons">"Buttons"</a>
                <a href="/component-library/inputs">"Inputs"</a>
            </nav>
            <Outlet/> // Allows nested routing for specific components
        </div>
    }
}
