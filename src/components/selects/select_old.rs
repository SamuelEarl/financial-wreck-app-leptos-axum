use leptos::prelude::*;
use leptos::logging::log;
use uuid::Uuid;
use stylance::*;

import_style!(css, "select_old.module.scss");

// 1. Context to share state between Trigger, Content, and Items
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

// 3. The Trigger (Button)
#[component]
pub fn SelectTrigger(children: Children) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectTrigger must be in <Select>");

    view! {
        <button
            class={css::select_trigger}
            popovertarget=move || ctx.popover_id.get().to_string()
        >
            {children()}
            <span class={css::select_trigger_arrow}>"▼"</span>
        </button>
    }
}

// 4. The Value Display
#[component]
pub fn SelectValue(#[prop(into)] placeholder: String) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectValue must be in <Select>");

    view! {
        <span>
            {move || {
                let label = ctx.selected_label.get();
                if label.is_empty() { placeholder.clone() } else { label }
            }}
        </span>
    }
}

// 5. The Content (The Popover)
#[component]
pub fn SelectContent(children: Children) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectContent must be in <Select>");

    view! {
        <div
            // Use the ID from context so it matches the Trigger
            id=move || ctx.popover_id.get().to_string()
            class={css::select_content}
            popover="auto" 
        >
            {children()}
        </div>
    }
}

// 6. Grouping (Optional wrapper)
#[component]
pub fn SelectGroup(children: Children) -> impl IntoView {
    view! { <div class={css::select_group}>{children()}</div> }
}

// 7. Label for a Group
#[component]
pub fn SelectGroupLabel(children: Children) -> impl IntoView {
    view! { 
        <div class={css::select_group_label}>
            {children()}
        </div> 
    }
}

// 8. The Item Wrapper
#[component]
pub fn SelectItem(children: Children) -> impl IntoView {
    view! {
        <div class={css::select_item_wrapper}>
            {children()}
        </div>
    }
}

// 9. The Actual Option Logic
#[component]
pub fn SelectOption(
    #[prop(into)] value: String,
    #[prop(optional, into)] label: Option<String>, 
    children: Children
) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectOption must be in <Select>");
    
    // ... existing logic ...
    let value_clone = value.clone(); // Re-clone for the closure if needed
    let display_label = label.unwrap_or(value.clone());

    let trigger_action = move |_| {
        ctx.set_selected_value.set(value.clone());
        ctx.set_selected_label.set(display_label.clone());
    };

    view! {
        <button
            class={css::select_option}
            on:click=trigger_action
            // Target the dynamic ID so this button can close the specific popover
            popovertarget=move || ctx.popover_id.get().to_string()
            popovertargetaction="hide"
        >
            {children()}
        </button>
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

    view! {
        <SelectRoot on_change=update_label>
            <SelectTrigger>
                <SelectValue placeholder=placeholder />
            </SelectTrigger>
            
            <SelectContent>
                <SelectGroup>
                    // Access the signal instead of the raw String.
                    <Show when=move || !label_sig.get().is_empty()>
                        <SelectGroupLabel>{move || label_sig.get()}</SelectGroupLabel>
                    </Show>
                    
                    // Iterate over the vector to create items
                    {options.into_iter().map(|opt| {
                        // We must clone strings to move them into the view closure
                        let val = opt.value.clone();
                        let lbl = opt.label.clone();
                        
                        view! {
                            <SelectItem>
                                // We pass a click handler inside SelectOption (see subcomponent above)
                                // that updates the context.
                                // NOTE: In the subcomponent implementation, you should add:
                                // `ctx.set_selected_label.set("My Label")` inside the click handler
                                // so the trigger updates.
                                <SelectOption 
                                    value=val
                                    label=lbl.clone()
                                >
                                    {lbl}
                                </SelectOption>
                            </SelectItem>
                        }
                    }).collect::<Vec<_>>()}
                    
                </SelectGroup>
            </SelectContent>
        </SelectRoot>
    }
}
