// This attribute ensures this entire module is ignored 
// if the "docs" feature is missing.
#![cfg(feature = "docs")]

use leptos::prelude::*;
use markdown_view_leptos::markdown_view;

// 1. IMPORT THE COMPONENT! 
// Even though we don't use <Alert> explicitly in the view! below,
// the markdown_view! macro will generate code that uses it.
use crate::components::inputs::input::{Input, CurrencyInput};

#[component]
pub fn InputsDocs() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (age, set_age) = signal("".to_string());
    let (price_cents, set_price_cents) = signal(1000); // Represents $10.00

    view! {
        <div class="docs-container">
            // 2. Load the file relative to the project root.
            // The macro parses the Markdown and injects the <Component> 
            // where the {{ ... }} tag is.
            {markdown_view!(file = "src/docs/inputs_docs.md")}
        </div>
    }
}
