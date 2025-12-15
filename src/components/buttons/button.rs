use leptos::prelude::*;
use leptos::attr::any_attribute::AnyAttribute;
use stylance::*;

import_style!(css, "button.module.scss");

use crate::components::colors_and_sizes::{
    BtnVariant, 
    Colors, 
    Sizes, 
    ElementWidths, 
    get_btn_colors, 
    get_element_sizes, 
    get_element_width
};

#[component]
pub fn Button(
    #[prop(default = BtnVariant::Primary)] variant: BtnVariant,
    #[prop(default = false)] inverted: bool,
    #[prop(default = None)] colors: Option<Colors>,
    #[prop(default = None)] sizes: Option<Sizes>,
    #[prop(default = ElementWidths::Auto)] width: ElementWidths,
    /// This captures all the other attributes that are not specifically defined on this component.
    #[prop(attrs)] attributes: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class={css::btn}
            style=format!("{} {} {}", get_btn_colors(colors, variant, inverted), get_element_sizes(sizes, true).all, get_element_width(width))
            // attr:disabled=(move || disabled || form_is_valid())
            // This spreads the attributes that are captured in the `attributes` prop,
            // including any on:<event> attributes.
            {..attributes}
        >
            {children()}
        </button>
    }
}
