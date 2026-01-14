# Radio Buttons

---

{{
    <label>"Select your favorite language"
        <RadioButtons 
            group_name="languages"
            options=programming_languages
            default_value="mojo"
            on_change=move |val| set_favorite.set(val)
        />
    </label>

    <br/>

    <p>"Currently selected: " {move || favorite.get()}</p>
}}

<br/>

```rust
use leptos::prelude::*;
use leptos::logging::log;

use crate::components::{
    radio_buttons::radio_buttons::{RadioButtonData, RadioButtons},
};

#[component]
pub fn PageComponent() -> impl IntoView {
    let programming_languages = vec![
        RadioButtonData { value: "mojo".into(), label: "Mojo".into() },
        RadioButtonData { value: "rust".into(), label: "Rust".into() },
        RadioButtonData { value: "python".into(), label: "Python".into() },
        RadioButtonData { value: "typescript".into(), label: "Typescript".into() },
    ];

    let (favorite, set_favorite) = signal("".to_string());

    view! {
        <label>"Select your favorite language"
            <RadioButtons 
                group_name="languages"
                options=programming_languages
                default_value="mojo"
                // Sync the internal state back to the parent
                on_change=move |val| set_favorite.set(val)
            />
        </label>

        <br/>

        <p>"Currently selected: " {move || favorite.get()}</p>
    }
}
```
