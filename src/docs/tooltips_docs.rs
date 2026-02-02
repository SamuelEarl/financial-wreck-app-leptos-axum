// // This attribute ensures this entire module is ignored 
// // if the "docs" feature is missing.
// #![cfg(feature = "docs")]

// use leptos::prelude::*;
// use markdown_view_leptos::markdown_view;

// // 1. IMPORT THE COMPONENT! 
// // Even though we don't use <Alert> explicitly in the view! below,
// // the markdown_view! macro will generate code that uses it.
// use crate::components::tooltips::tooltip::Tooltip;

// #[component]
// pub fn TooltipsDocs() -> impl IntoView {
//     view! {
//         <div class="docs-container">
//             // 2. Load the file relative to the project root.
//             // The macro parses the Markdown and injects the <Component> 
//             // where the {{ ... }} tag is.
//             {markdown_view!(file = "src/docs/tooltips_docs.md")}
//         </div>
//     }
// }
