use leptos::prelude::*;
use stylance::*;

import_style!(css, "dialog.module.scss");

// 1. CONTEXT
// This allows the Trigger to tell the Content to open/close
#[derive(Clone, Copy)]
struct DialogContext {
    is_open: Signal<bool>,
    set_is_open: WriteSignal<bool>,
}

// 2. ROOT COMPONENT
#[component]
pub fn Dialog(children: Children) -> impl IntoView {
    // We hold the state here
    let (is_open, set_is_open) = signal(false);

    // We provide the state to all children via Context
    provide_context(DialogContext { is_open: is_open.into(), set_is_open });

    // Handle body scroll locking
    Effect::new(move |_| {
        if is_open.get() {
            if let Some(doc) = document().body() {
                let _ = doc.style().set_property("overflow", "hidden");
            }
        } else {
            if let Some(doc) = document().body() {
                let _ = doc.style().remove_property("overflow");
            }
        }
    });

    view! {
        {children()}
    }
}

// 3. TRIGGER (The button that opens it)
#[component]
pub fn DialogTrigger(children: Children) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogTrigger must be inside <Dialog/>");

    view! {
        <div 
            class={css::trigger} 
            on:click=move |_| ctx.set_is_open.set(true)
        >
            {children()}
        </div>
    }
}

// 4. CONTENT (The Modal itself)
#[component]
pub fn DialogContent(
    children: ChildrenFn,
    #[prop(optional, into)] class: String, // Allow custom classes like width
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogContent must be inside <Dialog/>");

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
                </div>
            </div>
        </Show>
    }
}

// 5. BODY & SCROLL AREA
#[component]
pub fn DialogBody(children: Children) -> impl IntoView {
    view! { <div class={css::body}>{children()}</div> }
}

#[component]
pub fn ScrollArea(
    children: Children, 
    #[prop(optional, into)] class: String
) -> impl IntoView {
    view! { <div class=format!("{} {}", css::scroll_area, class)>{children()}</div> }
}

// 6. HEADER COMPONENTS
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

// 7. FOOTER & CLOSE BUTTON
#[component]
pub fn DialogFooter(children: Children) -> impl IntoView {
    view! { <div class={css::footer}>{children()}</div> }
}

#[component]
pub fn DialogClose(children: Children) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogClose must be inside <Dialog/>");

    view! {
        <div on:click=move |_| ctx.set_is_open.set(false)>
            {children()}
        </div>
    }
}
