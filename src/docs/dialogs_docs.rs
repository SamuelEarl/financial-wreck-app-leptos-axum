use leptos::prelude::*;
use markdown_view_leptos::markdown_view;

// 1. IMPORT THE COMPONENT! 
// Even though we don't use <Alert> explicitly in the view! below,
// the markdown_view! macro will generate code that uses it.
use crate::components::{
    colors_and_sizes::{
        BtnVariant,
        Colors,
    },
    buttons::button::Button,
};
use crate::components::dialogs::dialog::{ 
    Dialog, DialogTrigger, DialogContent, DialogBody, DialogHeader, 
    DialogTitle, DialogDescription, DialogFooter, DialogClose
};

#[component]
pub fn DialogsDocs() -> impl IntoView {
    view! {
        <div class="docs-container">
            // 2. Load the file relative to the project root.
            // The macro parses the Markdown and injects the <PrimaryButton> 
            // where the {{ ... }} tag is.
            {markdown_view!(file = "src/docs/dialogs_docs.md")}
        </div>
    }
}
