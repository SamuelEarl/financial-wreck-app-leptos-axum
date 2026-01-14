use leptos::prelude::*;
use markdown_view_leptos::markdown_view;
use leptos::logging::log;

// 1. IMPORT THE COMPONENT! 
// Even though we don't use <Alert> explicitly in the view! below,
// the markdown_view! macro will generate code that uses it.
use crate::components::{
    radio_buttons::radio_buttons::{
        RadioButtons,
        RadioButtonData,
        RadioGroup,
        RadioButton
    },
};

#[component]
pub fn RadioButtonsDocs() -> impl IntoView {
    // let programming_languages = vec![
    //     RadioButtonData { value: "mojo", label: "Mojo" },
    //     RadioButtonData { value: "rust", label: "Rust" },
    //     RadioButtonData { value: "typescript", label: "Typescript" },
    //     RadioButtonData { value: "python", label: "Python" },
    // ];

    // let (favorite, set_favorite) = signal("rust".to_string());

    view! {
        <div class="docs-container">
            // 2. Load the file relative to the project root.
            // The macro parses the Markdown and injects the <Component> 
            // where the {{ ... }} tag is.
            {markdown_view!(file = "src/docs/radio_buttons_docs.md")}
        </div>
    }
}
