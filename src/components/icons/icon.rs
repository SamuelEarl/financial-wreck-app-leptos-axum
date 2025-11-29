// Refer to the `generate-css-icons.js` script file in the root directory for instructions on how to use this component with CSS icons from Iconify.

// Usage Example:
// <Icon
//     icon="mdi:home".to_string()
//     // The "attr:" prefix ensures that the attribute goes into the "attributes" Vec.
//     attr:style="font-size: var(--size-8); color: red"
//     attr:id="menu-icon"
//     attr:data-role="nav"
//     attr:title="Menu"
//     // You can do this for events too.
//     attr:on:click=|_| println!("Clicked!")
// />

use leptos::prelude::*;
use leptos::attr::any_attribute::AnyAttribute;
use stylance::*;

import_style!(css, "icon.module.scss");

#[component]
pub fn Icon(
    icon: String,
    /// This captures all the other attributes that are not specifically defined on this component.
    #[prop(attrs)]
    attributes: Vec<AnyAttribute>,
) -> impl IntoView {
    let (icon_set, icon_name) = icon.split_once(":").unwrap();

    view! {
        <span
            class=format!("icon--{icon_set} icon--{icon_set}--{icon_name}")
            // This spreads the attributes that are captured in the `attributes` prop.
            {..attributes}
        ></span>
    }
}
