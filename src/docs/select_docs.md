# Select

`<select>` elements have historically been difficult to style while maintaining accessibility. There has been progress toward styling native `<select>` elements (see https://una.im/select-updates/), but there are still limitations when using those `<select>` elements on a mobile device. Ideally, when a user presses a `<select>` element on a mobile device, the dropdown/popover element will open up in full screen mode to make it easier to select an option without "fat fingering" the wrong option. Unfortunately the updated styling options do not do that on mobile (at least not right now).

This `<Select>` component, however, uses the Popover API (for improved accessibility) and uses a modal style popover to be user friendly on both desktop and mobile versions.

---

## Normal options (i.e. ungrouped)

{{
    <Select
        options=dinosaurs1
        placeholder="-- Select A Dinosaur --"
        on_change=Callback::new(move |val: String| {
            log!("Selected: {}", val);
            set_selected_dino1.set(val);
        })
    />

    <br/>

    <div>"Selected Dinosaur: " { move || selected_dino1.get() }</div>
}}

<br/>

```rust
use leptos::prelude::*;
use leptos::logging::log;

use crate::components::{
    colors_and_sizes::{Sizes},
    selects::select::{Select, OptionData},
};

#[component]
pub fn PageComponent() -> impl IntoView {
    let dinosaurs1 = vec![
        OptionData { group: None, value: "tyrannosaurus".to_string(), label: "Tyrannosaurus".to_string(), },
        OptionData { group: None, value: "velociraptor".to_string(), label: "Velociraptor".to_string(), },
        OptionData { group: None, value: "diplodocus".to_string(), label: "Diplodocus".to_string(), },
        OptionData { group: None, value: "saltasaurus".to_string(), label: "Saltasaurus".to_string(), },
        OptionData { group: None, value: "deinonychus".to_string(), label: "Deinonychus".to_string(), },
        OptionData { group: None, value: "apatosaurus".to_string(), label: "Apatosaurus".to_string(), },
    ];

    let (selected_dino1, set_selected_dino1) = signal("".to_string());

    view! {
        <Select
            options=dinosaurs1
            placeholder="-- Select A Dinosaur --"
            on_change=Callback::new(move |val: String| {
                log!("Selected: {}", val);
                set_selected_dino1.set(val);
            })
        />

        <br/>

        <div>"Selected Dinosaur: " { move || selected_dino1.get() }</div>
    }
}
```

<br/>
<br/>

## `default_value`, `<label>`, `btn_sizes`

If you pass a `default_value`, then the `placeholder` value will never be shown. It is best to use a `<label>` in scenarios like these. You can simply wrap the `<Select>` component inside a `<label>` element and add the label text in between quotes.

You can customize the size of the select button (trigger) by passing a `Sizes` struct to the `btn_sizes` prop.

{{
    <label>
        "Select A Dinosaur"
        <Select
            options=dinosaurs2
            default_value="tyrannosaurus".to_string()
            btn_sizes=Some(Sizes {
                pv: Some(2),
                ph: Some(3),
                ..Default::default()
            })
            on_change=Callback::new(move |val: String| {
                log!("Selected: {}", val);
                set_selected_dino2.set(val);
            })
        />
    </label>

    <br/>

    <div>"Selected Dinosaur: " { move || selected_dino2.get() }</div>
}}

<br/>

```rust
<label>
    "Select A Dinosaur"
    <Select
        options=dinosaurs2
        default_value="tyrannosaurus".to_string()
        btn_sizes=Some(Sizes {
            pv: Some(2),
            ph: Some(3),
            ..Default::default()
        })
        on_change=Callback::new(move |val: String| {
            log!("Selected: {}", val);
            set_selected_dino2.set(val);
        })
    />
</label>

<br/>

<div>"Selected Dinosaur: " { move || selected_dino2.get() }</div>
```

<br/>

## Grouped options (`<optgroup>`)

If you want to group the options in the popover (with `<optgroup>` elements), then you can add data to the `group` property. 

NOTE: For grouped options (`<optgroup>`), the order of the `<optgroup>`s will be the same as the `group` property in the options vector that is passed to the `options` prop. In other words, the first group that appears in the vector will be the first `<optgroup>` in the dropdown, the second group that appears will be the second `<optgroup>`, and so on.

{{
    <label>
        "Select A Dinosaur"
        <Select
            options=grouped_dinosaurs
            default_value="tyrannosaurus".to_string()
            btn_sizes=Some(Sizes {
                pv: Some(2),
                ph: Some(3),
                ..Default::default()
            })
            on_change=Callback::new(move |val: String| {
                log!("Selected: {}", val);
                set_selected_grouped_dino.set(val);
            })
        />
    </label>
}}

<br />

```rust
#[component]
pub fn PageComponent() -> impl IntoView {
    let grouped_dinosaurs = vec![
        OptionData { group: Some("theropods".to_string()), value: "tyrannosaurus".to_string(), label: "Tyrannosaurus".to_string(), },
        OptionData { group: Some("theropods".to_string()), value: "velociraptor".to_string(), label: "Velociraptor".to_string(), },
        OptionData { group: Some("theropods".to_string()), value: "diplodocus".to_string(), label: "Diplodocus".to_string(), },
        OptionData { group: Some("sauropods".to_string()), value: "saltasaurus".to_string(), label: "Saltasaurus".to_string(), },
        OptionData { group: Some("sauropods".to_string()), value: "deinonychus".to_string(), label: "Deinonychus".to_string(), },
        OptionData { group: Some("sauropods".to_string()), value: "apatosaurus".to_string(), label: "Apatosaurus".to_string(), },
    ];

    let (selected_grouped_dino, set_selected_grouped_dino) = signal("".to_string());

    view! {
        <Select
            options=grouped_dinosaurs
            default_value="tyrannosaurus".to_string()
            placeholder="Select A Dinosaur"
            btn_sizes=Some(Sizes {
                pv: Some(2),
                ph: Some(3),
                ..Default::default()
            })
            on_change=Callback::new(move |val: String| {
                log!("Selected: {}", val);
                set_selected_grouped_dino.set(val);
            })
        />

        <br/>

        <div>"Selected Dinosaur: " { move || selected_grouped_dino.get() }</div>
    }
}
```
