use leptos::prelude::*;
use stylance::*;

import_style!(css, "accordion.module.scss");

// 1. Context to manage state
// We use a tuple struct or simple struct to hold the signal.
// Clone/Copy is efficient for signals in 0.8.
#[derive(Clone, Copy, Debug)]
struct AccordionGroupContext {
    active_id: RwSignal<Option<String>>,
}

/// WRAPPER: Ensures mutually exclusive opening of items.
#[component]
pub fn AccordionGroup(children: Children) -> impl IntoView {
    // Create the reactive signal for the active ID
    let active_id = RwSignal::new(None::<String>);
    
    // Provide it to the context so children can find it
    provide_context(AccordionGroupContext { active_id });

    view! {
        <div class={css::accordion_group}>
            {children()}
        </div>
    }
}

/// ITEM: The collapsible unit.
#[component]
pub fn AccordionItem(
    /// Unique ID for this item (required).
    #[prop(into)] id: String,
    
    /// The visible header text.
    #[prop(into)] title: String,
    
    /// The content to hide/show.
    children: Children,
) -> impl IntoView {
    // 1. Try to get the Group Context
    let group_context = use_context::<AccordionGroupContext>();

    // 2. Create a local fallback signal (for standalone mode)
    let local_is_open = RwSignal::new(false);

    // 3. Derived Signal: Am I open?
    // We calculate this dynamically based on whether we are in a group or not.
    let id_clone = id.clone();
    let is_open = move || {
        if let Some(ctx) = group_context {
            // Group Mode: Check if my ID matches the global active ID
            ctx.active_id.get() == Some(id_clone.clone())
        } else {
            // Standalone Mode: Check my local boolean
            local_is_open.get()
        }
    };

    // 4. Click Handler
    let toggle = move |_| {
        if let Some(ctx) = group_context {
            // Group Mode Logic
            let current = ctx.active_id.get();
            if current == Some(id.clone()) {
                ctx.active_id.set(None); // Close if clicking self
            } else {
                ctx.active_id.set(Some(id.clone())); // Open self
            }
        } else {
            // Standalone Mode Logic
            local_is_open.update(|b| *b = !*b);
        }
    };

    view! {
        <div class={css::accordion_item}>
            // Accordion Item Button
            <button
                type="button"
                on:click=toggle
                class={css::accordion_item_btn}
            >
                <span>{title}</span>
                
                // Chevron Icon (Rotates when open)
                <svg
                    width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                    stroke-linecap="round" stroke-linejoin="round"
                    style:transition="transform 0.2s ease"
                    // Dynamic Style: Rotate based on state
                    style:transform={
                        let is_open_clone = is_open.clone();
                        move || if is_open_clone() { "rotate(180deg)" } else { "rotate(0deg)" }
                    }
                >
                    <path d="M19 9l-7 7-7-7" />
                </svg>
            </button>

            // Accordion Item Panel
            <div
                class={css::accordion_item_panel_animation_wrapper}
                // Dynamic Style: Toggle Display
                // Dynamic Value: 1fr = open, 0fr = closed
                style:grid-template-rows=move || if is_open() { "1fr" } else { "0fr" }
            >
                <div class={css::accordion_item_panel_outer}>
                    <div class={css::accordion_item_panel_inner}>
                        {children()}
                    </div>
                </div>
            </div>
        </div>
    }
}
