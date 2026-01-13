use leptos::prelude::*;
// use leptos::logging::log;
// use uuid::Uuid;
use stylance::*;

import_style!(css, "radio_buttons.module.scss");

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

    let val_clone = value.clone();
    let is_checked = move || ctx.selected.get() == val_clone;

    view! {
        <div class={css::radio_label_wrapper}>
            <label class={css::radio_label}>
                <input
                    type="radio"
                    class={css::radio_input}
                    name=ctx.name.clone()
                    value=value.clone()
                    prop:checked=is_checked
                    on:change=move |_| ctx.set_selected.set(value.clone())
                /> {label}
                <span class={css::radio_checkmark}></span>
            </label>
        </div>
    }
}
