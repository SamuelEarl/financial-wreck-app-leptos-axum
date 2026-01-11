use leptos::prelude::*;
use leptos::logging::log;
use uuid::Uuid;
use stylance::*;

import_style!(css, "select.module.scss");

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
        <select>
            // <option value="first">"First"</option>
            // <option value="second">"Second"</option>
            // <option value="third">"Third"</option>
            <optgroup label="Top Group">
                <option value="first">"First"</option>
                <option value="second">"Second"</option>
                <option value="third">"Third"</option>
            </optgroup>
            <optgroup label="Bottom Group">
                <option value="fourth">"Fourth"</option>
                <option value="fifth">"Fifth"</option>
                <option value="sixth">"Sixth"</option>
            </optgroup>
        </select>
    }
}
