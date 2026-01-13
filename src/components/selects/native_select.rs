use leptos::prelude::*;
use leptos::logging::log;
use uuid::Uuid;
use stylance::*;

import_style!(css, "native_select.module.scss");

// 1. The Standard Data Structure
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OptionData {
    pub group: Option<String>, // Defaults to None.
    pub value: String,
    pub label: String,
}

// 2. The Reusable Component
#[component]
pub fn Select() -> impl IntoView {
    view! {
        <select class={css::native_select}>
            <button class={css::select_button}>
                <selectedcontent></selectedcontent>
                <span class="arrow"></span>
            </button>
            // <option value="first">"First"</option>
            // <option value="second">"Second"</option>
            // <option value="third">"Third"</option>
            <optgroup label="Top Group" class={css::native_optgroup}>
                <option value="first" class={css::native_option}>"First"</option>
                <option value="second" class={css::native_option}>"Second"</option>
                <option value="third" class={css::native_option}>"Third"</option>
            </optgroup>
            <optgroup label="Bottom Group" class={css::native_optgroup}>
                <option value="fourth" class={css::native_option}>"Fourth"</option>
                <option value="fifth" class={css::native_option}>"Fifth"</option>
                <option value="sixth" class={css::native_option}>"Sixth"</option>
            </optgroup>
        </select>
    }
}
