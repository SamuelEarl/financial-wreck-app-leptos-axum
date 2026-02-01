use leptos::prelude::*;
use leptos::attr::any_attribute::AnyAttribute;
use stylance::*;

use crate::utils::format_currency::format_currency;

import_style!(css, "input.module.scss");

#[component]
pub fn Input(
    /// This attribute allows you to pass any standard HTML attributes
    /// such as type, placeholder, disabled, etc.
    #[prop(attrs)] attributes: Vec<AnyAttribute>,
) -> impl IntoView {
    view! {
        <input
            class={css::input}
            // This spreads all the passed attributes onto the underlying element
            {..attributes}
        />
    }
}


#[component]
pub fn CurrencyInput(
    /// The value in CENTS (e.g., 1050 = $10.50)
    #[prop(into)] value: Signal<i64>,
    /// Setter for the value in CENTS
    #[prop(into)] set_value: WriteSignal<i64>,
    /// Placeholder text
    #[prop(optional, into)] placeholder: String,
    /// Any extra attributes (class, id, disabled, etc.)
    #[prop(attrs)] attributes: Vec<AnyAttribute>,
) -> impl IntoView {
    // 1. Local state to track focus
    let (is_focused, set_focused) = signal(false);

    // 2. Local buffer for what is inside the input box.
    // We use this to preserve trailing decimals (like "10.") while typing.
    let (input_buffer, set_input_buffer) = signal("".to_string());

    // 3. SYNC LOGIC: Keep the input buffer in sync with external value changes
    // But ONLY if the user is not currently editing it.
    Effect::new(move |_| {
        if !is_focused.get() {
            // This will now show $0.00 instead of an empty string
            set_input_buffer.set(format_currency(value.get(), None));
        }
    });

    view! {
        <input
            class={css::input}

            // Splat generic attributes first
            {..attributes}
            
            type="text"
            placeholder=placeholder
            
            // Bind the DOM value to our local buffer
            prop:value=move || input_buffer.get()
            
            // --- EVENT HANDLERS --- //

            // ON FOCUS: Switch to "Edit Mode" (Decimal format, no symbols)
            on:focus=move |_| {
                set_focused.set(true);
                let cents = value.get();
                if cents == 0 {
                    // Clear the "$0.00" so the placeholder appears and typing is clean
                    set_input_buffer.set("".to_string());
                } else {
                    let decimal_val = (cents as f64) / 100.0;
                    set_input_buffer.set(format!("{:.2}", decimal_val));
                }
            }
            
            // ON BLUR: Switch to "View Mode" (Formatted currency)
            on:blur=move |_| {
                set_focused.set(false);
                // The Effect::new above will automatically trigger 
                // and format the value back to "$1,050.00"
            }
            
            // ON INPUT: Handle the math
            on:input=move |event| {
                let raw_val = event_target_value(&event);
                
                // 1. Update the display buffer immediately (so user sees what they type)
                set_input_buffer.set(raw_val.clone());
                
                // 2. Parse and update the parent i64 signal
                // Remove non-numeric chars except dot
                let clean_val = raw_val
                    .replace(|c: char| !c.is_digit(10) && c != '.', "")
                    .trim()
                    .to_string();

                if let Ok(parsed_float) = clean_val.parse::<f64>() {
                    // Math: 10.50 * 100 = 1050.0
                    // Use round() to fix floating point drift (e.g. 1049.99999).
                    // When dealing with float-to-int conversion, always round. 19.99 * 100.0 might equal 1998.9999999 in float math. Casting directly as i64 would truncate it to 1998 (losing a penny). round() ensures it snaps correctly to 1999.
                    let cents = (parsed_float * 100.0).round() as i64;
                    set_value.set(cents);
                } else if clean_val.is_empty() {
                    set_value.set(0);
                }
            }
        />
    }
}
