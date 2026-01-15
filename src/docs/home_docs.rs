// This attribute ensures this entire module is ignored 
// if the "docs" feature is missing.
#![cfg(feature = "docs")]

use leptos::prelude::*;
use markdown_view_leptos::markdown_view;

// 1. IMPORT THE COMPONENTS THAT ARE REFERENCED IN THE MARKDOWN FILE! 
// Even though we don't use <Alert> explicitly in the view! below,
// the markdown_view! macro will generate code that uses it.
// use crate::components::accordions::accordion::{
//     AccordionGroup,
//     AccordionItem,
// };

#[component]
pub fn HomeDocs() -> impl IntoView {
    view! {
        <div class="docs-container">
            // 2. Load the file relative to the project root.
            // The macro parses the Markdown and injects the <Component> 
            // where the {{ ... }} tag is.
            {markdown_view!(file = "src/docs/home_docs.md")}
        </div>
    }
}
