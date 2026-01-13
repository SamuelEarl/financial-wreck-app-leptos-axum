use leptos::prelude::*;
use stylance::*;

use crate::components::{
    colors_and_sizes::{BtnVariant, Colors, Sizes, ElementWidths},
    buttons::button::Button,
};

import_style!(css, "select.module.scss");

// 1. The Standard Data Structure
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OptionData {
    pub group: Option<String>, // Defaults to None.
    pub value: String,
    pub label: String,
}

// CONTEXT
// This allows the Trigger to tell the Content to open/close
#[derive(Clone, Copy)]
struct SelectContext {
    is_open: Signal<bool>,
    set_is_open: WriteSignal<bool>,
}

// ROOT COMPONENT
#[component]
pub fn Select(children: Children) -> impl IntoView {
    // The `is_open` state is held here.
    let (is_open, set_is_open) = signal(false);

    // The `is_open` state is provided to all children via Context.
    provide_context(SelectContext { is_open: is_open.into(), set_is_open });

    // Handle body scroll locking
    // This adds the CSS rule overflow: hidden; to the <body> tag. This cuts off any content that goes outside the screen edges and removes the scrollbars, which prevents the user from scrolling the page content.
    // NOTE: This doesn't actually seem to do anything, but I am leaving it here in case I run into scrolling issues later.
    // Effect::new(move |_| {
    //     if is_open.get() {
    //         if let Some(doc) = document().body() {
    //             let _ = doc.style().set_property("overflow", "hidden");
    //         }
    //     } else {
    //         if let Some(doc) = document().body() {
    //             let _ = doc.style().remove_property("overflow");
    //         }
    //     }
    // });

    view! {
        {children()}
    }
}


// TRIGGER (The button that opens it)
#[component]
pub fn SelectText(children: Children) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectText must be inside <Select/>");

    view! {
        // TODO: Should I replace this <Button> component with a plain button that use standard colors and that can take customizable size props? I could also set `text-align: left` without messing with the <Button> component.
        <Button
            colors=Some(Colors {
                bg: "var(--warning-bg)".to_string(),
                fg: "var(--warning-fg)".to_string(),
                br: "var(--warning-bg)".to_string(),
                ol: "var(--warning-bg)".to_string(),
            })
            sizes=Some(Sizes {
                pv: Some(0),
                ph: Some(2),
                ..Default::default()
            })
            width={ElementWidths::Full}
            on:click=move |_| ctx.set_is_open.set(true)
        >
            {children()}
        </Button>
    }
}

// CONTENT (The Modal itself)
#[component]
pub fn SelectOptions(
    children: ChildrenFn,
    #[prop(optional, into)] class: String, // Allow custom classes like width.
) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectOptions must be inside <Select/>");

    view! {
        <Show when=move || ctx.is_open.get()>
            // Backdrop
            <div 
                class={css::backdrop} 
                on:click=move |_| ctx.set_is_open.set(false)
            >
                // Modal Window
                <div 
                    class=format!("{} {}", css::content, class)
                    // Prevent clicking the modal from closing it
                    on:click=move |e| e.stop_propagation() 
                    role="dialog"
                    aria-modal="true"
                >
                    {children.clone()()}
                    // TODO: Replace options with radio buttons.
                    // Replace optgroups with labels.
                    <div class={css::optgroup}>"Top Group"
                        <div class={css::option}>"First"</div>
                        <div class={css::option}>"Second"</div>
                        <div class={css::option}>"Third"</div>
                    </div>
                    <div class={css::optgroup}>"Bottom Group"
                        <div class={css::option}>"Fourth"</div>
                        <div class={css::option}>"Fifth"</div>
                        <div class={css::option}>"Sixth"</div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
