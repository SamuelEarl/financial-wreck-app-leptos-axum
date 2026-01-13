use leptos::prelude::*;
use stylance::*;

use crate::components::{
    colors_and_sizes::{Sizes, get_element_sizes},
};

import_style!(css, "select_alt1.module.scss");

// 1. Context to share state between Trigger, Content, and Items
// This allows the Trigger to tell the Content to open/close
#[derive(Clone, Copy)]
struct SelectContext {
    selected_value: Signal<String>,
    set_selected_value: WriteSignal<String>,
    // We store the label too so the Trigger knows what to display
    selected_label: Signal<String>,
    set_selected_label: WriteSignal<String>,
    // Controls the native popover state
    is_open: Signal<bool>,
    set_is_open: WriteSignal<bool>,
    popover_id: Signal<Uuid>,
    set_popover_id: WriteSignal<Uuid>,
}

// 2. The Root Component
#[component]
pub fn SelectRoot(
    // Optional: Default value
    #[prop(optional, into)] default_value: Option<String>,
    // Optional: Callback when selection changes
    #[prop(optional, into)] on_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let (selected_value, set_selected_value) = signal(default_value.unwrap_or_default());
    let (selected_label, set_selected_label) = signal(String::new());
    let (is_open, set_is_open) = signal(false);
    let (popover_id, set_popover_id) = signal(Uuid::now_v7());

    // Provide context to all children
    provide_context(SelectContext {
        selected_value: selected_value.into(),
        set_selected_value,
        selected_label: selected_label.into(),
        set_selected_label,
        is_open: is_open.into(),
        set_is_open,
        popover_id: popover_id.into(), 
        set_popover_id,
    });

    // Effect to run the on_change callback
    Effect::new(move |_| {
        if let Some(cb) = on_change {
            cb.run(selected_value.get());
        }
    });

    view! {
        <div class={css::select_root}>
            {children()}
        </div>
    }
}


// TRIGGER (The button that opens it)
#[component]
pub fn SelectButton(
    #[prop(default = None)] sizes: Option<Sizes>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectButton must be inside <Select/>");

    view! {
        <button
            class={css::select_button}
            style=format!("{}", get_element_sizes(sizes, true).all)
            on:click=move |_| ctx.set_is_open.set(true)
        >
            <span class="selected_option">{children()}</span>
            <span class={css::arrow}></span>
        </button>
    }
}

// CONTENT (The Modal itself)
#[component]
pub fn SelectOptions(
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
                    // TODO: Replace options with radio buttons.
                    // Replace optgroups with labels.
                    // Use the OptionData struct with group, value, label properties.
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

// 1. The Standard Data Structure
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OptionData {
    pub group: Option<String>, // Defaults to None.
    pub value: String,
    pub label: String,
}

// 2. The Reusable Component
#[component]
pub fn Select(
    // Accepts any list of objects
    options: Vec<OptionData>,
    
    // Label for the group (optional)
    #[prop(optional, into)] label: String,
    
    // Placeholder text
    #[prop(optional, into)] placeholder: String,
    
    // Callback
    #[prop(optional, into)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    // Wrap the String in a Signal so it becomes Copy-able
    let (label_sig, _) = signal(label);
    
    // We define a listener to update the internal label when the value changes
    let update_label = {
        let options_map = options.clone();
        Callback::new(move |val: String| {
            // In a real app, you might sync this to the internal context
            if let Some(cb) = on_change {
                cb.run(val);
            }
        })
    };

    // // The `is_open` state is held here.
    // let (is_open, set_is_open) = signal(false);

    // // The `is_open` state is provided to all children via Context.
    // provide_context(SelectContext { is_open: is_open.into(), set_is_open });

    view! {
        <SelectRoot on_change=update_label>
            <SelectButton
                sizes=Some(Sizes {
                    pv: Some(2),
                    ph: Some(2),
                    ..Default::default()
                })
            >
                "Select An Option"
            </SelectButton>
        </SelectRoot>
    }
}
