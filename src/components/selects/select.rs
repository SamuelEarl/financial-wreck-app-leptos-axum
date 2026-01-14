use std::collections::HashMap;
use leptos::prelude::*;
// use leptos::logging::log;
use uuid::Uuid;
use stylance::*;

use crate::components::{
    colors_and_sizes::{Sizes, get_element_sizes},
};

import_style!(css, "select.module.scss");

#[derive(Debug)]
pub struct GroupedItem {
    pub value: String,
    pub label: String,
}

// 1. Context to share state between Button, Content, and Items
#[derive(Clone, Copy)]
struct SelectContext {
    selected_value: Signal<String>,
    set_selected_value: WriteSignal<String>,
    // We store the label too so the Button knows what to display
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
    default_value: Option<String>,
    initial_label: Option<String>,
    // Optional: Callback when selection changes
    #[prop(optional, into)] on_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let (selected_value, set_selected_value) = signal(default_value.unwrap_or_default());
    // Initialize label with the provided initial_label.
    let (selected_label, set_selected_label) = signal(initial_label.unwrap_or_default());
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

// 3. The Button
#[component]
pub fn SelectButton(
    #[prop(default = None)] btn_sizes: Option<Sizes>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectButton must be in <Select>");

    view! {
        <button
            class={css::select_button}
            style=format!("{}", get_element_sizes(btn_sizes, true).all)
            popovertarget=move || ctx.popover_id.get().to_string()
            role="combobox"
        >
            {children()}
            <span class={css::select_button_arrow}>"›"</span>
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
            // Use the ID from context so it matches the Button
            id=move || ctx.popover_id.get().to_string()
            class={css::select_content}
            popover="auto" 
            role="listbox"
        >
            {children()}
        </div>
    }
}

// 6. Grouping (Optional wrapper)
#[component]
pub fn OptGroup(children: Children) -> impl IntoView {
    view! { <div class={css::opt_group} role="group">{children()}</div> }
}

// 7. Label for a Group
#[component]
pub fn OptGroupLabel(children: Children) -> impl IntoView {
    view! { 
        <div class={css::opt_group_label}>
            {children()}
        </div> 
    }
}

// 8. The Item Wrapper
#[component]
pub fn SelectItem(children: Children) -> impl IntoView {
    view! {
        <div class={css::select_item}>
            {children()}
        </div>
    }
}

// 9. The Actual Option Logic
#[component]
pub fn SelectOption(
    #[prop(into)] value: String,
    #[prop(optional, into)] label: Option<String>, 
    has_opt_group_label: bool,
    children: Children
) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectOption must be in <Select>");
    let display_label = label.unwrap_or(value.clone());
    // Clone value once for the comparison closure.
    let current_val = value.clone();
    // This closure will re-run whenever ctx.selected_value changes.
    let is_selected = move || ctx.selected_value.get() == current_val;

    let button_action = move |_| {
        ctx.set_selected_value.set(value.clone());
        ctx.set_selected_label.set(display_label.clone());
    };

    view! {
        <button
            type="button"
            role="option"
            class={css::select_option}
            class=(css::has_opt_group_label, move || has_opt_group_label)
            // Add the selected class reactively
            class=(css::selected_option, move || is_selected())
            on:click=button_action
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
    #[prop(optional)] default_value: Option<String>,
    // Placeholder text
    #[prop(optional, into)] placeholder: String,
    #[prop(default = None)] btn_sizes: Option<Sizes>,
    // Callback
    #[prop(optional, into)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    // Find the label associated with the default value.
    let initial_label = default_value.as_ref().and_then(|def_val| {
        options.iter()
            .find(|opt| &opt.value == def_val)
            .map(|opt| opt.label.clone())
    });

    // We define a listener to update the button label when the value changes.
    let update_btn_label = {
        Callback::new(move |val: String| {
            // In a real app, you might sync this to the internal context
            if let Some(cb) = on_change {
                cb.run(val);
            }
        })
    };

    let mut ordered_groups: Vec<(String, Vec<GroupedItem>)> = Vec::new();

    for opt in options {
        let group_name = opt.group.unwrap_or_default();
        let item = GroupedItem {
            value: opt.value,
            label: opt.label,
        };

        // Find existing group or create a new one
        if let Some(pos) = ordered_groups.iter().position(|(name, _)| name == &group_name) {
            ordered_groups[pos].1.push(item);
        } else {
            ordered_groups.push((group_name, vec![item]));
        }
    }

    view! {
        <SelectRoot
            default_value=default_value 
            initial_label=initial_label
            on_change=update_btn_label
        >
            <SelectButton btn_sizes=btn_sizes>
                <SelectValue placeholder=placeholder />
            </SelectButton>
            
            <SelectContent>
                {ordered_groups.into_iter().map(|(group_name, items)| {
                    let is_grouped = !group_name.is_empty();
                    
                    // 1. Generate the list of items first
                    let items_view = items.into_iter().map(|item| {
                        view! {
                            <SelectItem>
                                <SelectOption
                                    value=item.value 
                                    label=item.label.clone()
                                    has_opt_group_label=is_grouped
                                >
                                    {item.label}
                                </SelectOption>
                            </SelectItem>
                        }
                    }).collect_view();

                    // 2. Conditionally wrap in OptGroup
                    if is_grouped {
                        view! {
                            <OptGroup>
                                <OptGroupLabel>{group_name}</OptGroupLabel> 
                                {items_view}
                            </OptGroup>
                        }.into_any()
                    } else {
                        // Render the items directly without the group wrappers
                        items_view.into_any()
                    }
                }).collect_view()}
            </SelectContent>
        </SelectRoot>
    }
}
