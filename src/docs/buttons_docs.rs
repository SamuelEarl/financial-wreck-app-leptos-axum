use leptos::prelude::*;
use markdown_view_leptos::markdown_view;
use leptos::logging::log;

// 1. IMPORT THE COMPONENT! 
// Even though we don't use <Alert> explicitly in the view! below,
// the markdown_view! macro will generate code that uses it.
use crate::components::{
    buttons::button::Button,
    colors_and_sizes::{
        BtnVariant, 
        Colors, 
        Sizes, 
        ElementWidths,
    }
};

#[component]
pub fn ButtonsDocs() -> impl IntoView {
    view! {
        <div class="docs-container">
            // 2. Load the file relative to the project root.
            // The macro parses the Markdown and injects the <Component> 
            // where the {{ ... }} tag is.
            {markdown_view!(file = "src/docs/buttons_docs.md")}
        </div>
    }
}
