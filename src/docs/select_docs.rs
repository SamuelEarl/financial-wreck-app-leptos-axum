// This attribute ensures this entire module is ignored 
// if the "docs" feature is missing.
#![cfg(feature = "docs")]

use leptos::prelude::*;
use markdown_view_leptos::markdown_view;
use leptos::logging::log;

// 1. IMPORT THE COMPONENT! 
// Even though we don't use <Alert> explicitly in the view! below,
// the markdown_view! macro will generate code that uses it.
use crate::components::{
    colors_and_sizes::{Sizes},
    selects::select::{Select, OptionData},
};

#[component]
pub fn SelectDocs() -> impl IntoView {
    let dinosaurs1 = vec![
        OptionData { group: None, value: "tyrannosaurus".to_string(), label: "Tyrannosaurus".to_string(), },
        OptionData { group: None, value: "velociraptor".to_string(), label: "Velociraptor".to_string(), },
        OptionData { group: None, value: "diplodocus".to_string(), label: "Diplodocus".to_string(), },
        OptionData { group: None, value: "saltasaurus".to_string(), label: "Saltasaurus".to_string(), },
        OptionData { group: None, value: "deinonychus".to_string(), label: "Deinonychus".to_string(), },
        OptionData { group: None, value: "apatosaurus".to_string(), label: "Apatosaurus".to_string(), },
    ];

    let (selected_dino1, set_selected_dino1) = signal("".to_string());

    let dinosaurs2 = vec![
        OptionData { group: None, value: "tyrannosaurus".to_string(), label: "Tyrannosaurus".to_string(), },
        OptionData { group: None, value: "velociraptor".to_string(), label: "Velociraptor".to_string(), },
        OptionData { group: None, value: "diplodocus".to_string(), label: "Diplodocus".to_string(), },
        OptionData { group: None, value: "saltasaurus".to_string(), label: "Saltasaurus".to_string(), },
        OptionData { group: None, value: "deinonychus".to_string(), label: "Deinonychus".to_string(), },
        OptionData { group: None, value: "apatosaurus".to_string(), label: "Apatosaurus".to_string(), },
    ];

    let (selected_dino2, set_selected_dino2) = signal("".to_string());

    let grouped_dinosaurs = vec![
        OptionData { group: Some("theropods".to_string()), value: "tyrannosaurus".to_string(), label: "Tyrannosaurus".to_string(), },
        OptionData { group: Some("theropods".to_string()), value: "velociraptor".to_string(), label: "Velociraptor".to_string(), },
        OptionData { group: Some("theropods".to_string()), value: "diplodocus".to_string(), label: "Diplodocus".to_string(), },
        OptionData { group: Some("sauropods".to_string()), value: "saltasaurus".to_string(), label: "Saltasaurus".to_string(), },
        OptionData { group: Some("sauropods".to_string()), value: "deinonychus".to_string(), label: "Deinonychus".to_string(), },
        OptionData { group: Some("sauropods".to_string()), value: "apatosaurus".to_string(), label: "Apatosaurus".to_string(), },
    ];

    let (_selected_grouped_dino, set_selected_grouped_dino) = signal("".to_string());

    view! {
        <div class="docs-container">
            // 2. Load the file relative to the project root.
            // The macro parses the Markdown and injects the <Component> 
            // where the {{ ... }} tag is.
            {markdown_view!(file = "src/docs/select_docs.md")}
        </div>
    }
}
