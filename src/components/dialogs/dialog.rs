use leptos::prelude::*;
use uuid::Uuid;
use stylance::*;

use crate::components::colors_and_sizes::{
    BtnVariant, 
    Colors, 
    Sizes, 
    ElementWidths, 
    get_btn_colors, 
    get_element_sizes, 
    get_element_width
};

import_style!(css, "dialog.module.scss");

// CONTEXT
// This allows the Trigger to tell the Content to open/close
#[derive(Clone, Copy)]
struct DialogContext {
    popover_id: Signal<String>,
}

// ROOT COMPONENT
#[component]
pub fn Dialog(children: Children) -> impl IntoView {
    // Generate a unique ID for the popover connection
    let (popover_id, _) = signal(format!("dialog-{}", Uuid::now_v7()));

    provide_context(DialogContext { popover_id: popover_id.into() });

    view! {
        {children()}
    }
}

// TRIGGER (The button that opens it)
#[component]
pub fn DialogTrigger(
    #[prop(default = BtnVariant::Primary)] variant: BtnVariant,
    #[prop(default = false)] inverted: bool,
    #[prop(default = None)] colors: Option<Colors>,
    #[prop(default = None)] sizes: Option<Sizes>,
    #[prop(default = ElementWidths::Auto)] width: ElementWidths,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("missing DialogContext");

    view! {
        // popovertarget links the button to the content ID
        <button 
            class={css::trigger} 
            style=format!("{} {} {}", get_btn_colors(colors, variant, inverted), get_element_sizes(sizes, true).all, get_element_width(width))
            popovertarget=move || ctx.popover_id.get()
        >
            {children()}
        </button>
    }
}

// CONTENT (The Modal itself)
#[component]
pub fn DialogContent(
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("missing DialogContext");

    view! {
        <div 
            // The magic happens here:
            id=move || ctx.popover_id.get()
            popover="auto" 
            role="dialog"
            class={css::content}
        >
            // With Popover API, the "backdrop" is handled via CSS ::backdrop
            {children.clone()()}
        </div>
    }
}

// HEADER COMPONENTS
#[component]
pub fn DialogHeader(children: Children) -> impl IntoView {
    view! { <div class={css::header}>{children()}</div> }
}

#[component]
pub fn DialogTitle(children: Children) -> impl IntoView {
    view! { <h2 class={css::title}>{children()}</h2> }
}

#[component]
pub fn DialogDescription(children: Children) -> impl IntoView {
    view! { <p class={css::description}>{children()}</p> }
}

// BODY
#[component]
pub fn DialogBody(children: Children) -> impl IntoView {
    view! { <div class={css::body}>{children()}</div> }
}

// FOOTER & CLOSE BUTTON
#[component]
pub fn DialogFooter(children: Children) -> impl IntoView {
    view! { <div class={css::footer}>{children()}</div> }
}

#[component]
pub fn DialogClose(
    #[prop(default = BtnVariant::Primary)] variant: BtnVariant,
    #[prop(default = false)] inverted: bool,
    #[prop(default = None)] colors: Option<Colors>,
    #[prop(default = None)] sizes: Option<Sizes>,
    #[prop(default = ElementWidths::Auto)] width: ElementWidths,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("missing DialogContext");

    view! {
        // Using popovertargetaction="hide" tells the browser to close the ID
        <button
            class={css::dialog_close}
            popovertarget=move || ctx.popover_id.get()
            popovertargetaction="hide"
            style=format!("{} {} {}", get_btn_colors(colors, variant, inverted), get_element_sizes(sizes, true).all, get_element_width(width))
        >
            {children()}
        </button>
    }
}
