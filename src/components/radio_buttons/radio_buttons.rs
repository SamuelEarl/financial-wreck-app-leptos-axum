use leptos::prelude::*;
// use leptos::logging::log;
// use uuid::Uuid;
use stylance::*;

import_style!(css, "radio_buttons.module.scss");

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RadioButtonData {
    pub value: String,
    pub label: String,
}

#[derive(Clone)]
struct RadioContext {
    name: String,
    selected: ReadSignal<String>,
    set_selected: WriteSignal<String>,
}

#[component]
pub fn RadioGroup(
    #[prop(into)] name: String,
    #[prop(into)] value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    children: Children,
) -> impl IntoView {    
    // Provide context so children don't need props passed manually
    provide_context(RadioContext {
        name,
        selected: value,
        set_selected: set_value,
    });

    view! {
        <div role="radiogroup" class={css::radio_group}>
            {children()}
        </div>
    }
}

#[component]
pub fn RadioButton(
    #[prop(into)] value: String,
    #[prop(into)] label: String,
) -> impl IntoView {
    let ctx = use_context::<RadioContext>()
        .expect("RadioButton must be used within a RadioGroup");

    // Memoize the checked state for better performance
    let is_checked = {
        let val = value.clone();
        move || ctx.selected.get() == val
    };

    let name = ctx.name.clone();
    let val_for_change = value.clone();

    view! {
        <div class={css::radio_label_wrapper}>
            <label class={css::radio_label}>
                <input
                    type="radio"
                    class={css::radio_input}
                    name=name
                    value=value
                    prop:checked=is_checked
                    on:change=move |_| ctx.set_selected.set(val_for_change.clone())
                /> {label}
                <span class={css::radio_checkmark}></span>
            </label>
        </div>
    }
}

#[component]
pub fn RadioButtons(
    #[prop(into)] group_name: String,
    options: Vec<RadioButtonData>,
    #[prop(optional, into)] default_value: Option<String>,
    // An optional callback for the parent to listen to
    #[prop(optional, into)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    // Initialize the signal with the default value.
    let (selected, set_selected) = signal(default_value.unwrap_or_default());

    // When the signal changes, we run the on_change callback if it exists.
    Effect::new(move |_| {
        if let Some(cb) = on_change {
            cb.run(selected.get());
        }
    });

    view! {
        <RadioGroup
            name=group_name
            value=selected
            set_value=set_selected
        >
            {options.into_iter().map(|opt| {
                view! {
                    <RadioButton value=opt.value label=opt.label />
                }
            }).collect_view()}
        </RadioGroup>
    }
}
