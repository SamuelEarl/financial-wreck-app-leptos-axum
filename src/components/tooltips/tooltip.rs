use leptos::prelude::*;
use leptos::html::Div;
use leptos::portal::Portal;
// use web_sys::wasm_bindgen::JsCast;
use stylance::*;
use leptos::logging::log;

import_style!(css, "tooltip.module.scss");

#[component]
pub fn Tooltip(
    #[prop(into)] text: String,
    children: Children,
) -> impl IntoView {
    let (is_visible, set_visible) = signal(false);
    let (coords, set_coords) = signal((0.0, 0.0));
    let (flip_bottom, set_flip_bottom) = signal(false);
    let trigger_ref = NodeRef::<Div>::new();

    let stored_text = StoredValue::new(text);

    let update_position = move || {
        if let Some(rect) = trigger_ref.get().map(|el| el.get_bounding_client_rect()) {
            let mut x = rect.left();
            let mut y = rect.top() - 10.0; // Assume a small height offset for now
            
            // If it's too high, flip to bottom
            if y < 20.0 {
                y = rect.bottom() + 10.0;
                set_flip_bottom.set(true);
            } else {
                set_flip_bottom.set(false);
            }
            set_coords.set((x, y));
        }
    };

    view! {
        <div 
            node_ref=trigger_ref
            on:mouseenter=move |_| { update_position(); set_visible.set(true); }
            on:mouseleave=move |_| set_visible.set(false)
            style="display: inline-block;"
        >
            {children()}

            <Show when=move || is_visible.get()>
                <Portal>
                    <div 
                        class={css::tooltip_bubble}
                        class:is_bottom=move || flip_bottom.get()
                        style=move || {
                            let (x, y) = coords.get();
                            format!(
                                "left: {}px; top: {}px; position: fixed; z-index: 999999; pointer-events: none; transform: {};", 
                                x, y, 
                                if flip_bottom.get() { "translateY(0)" } else { "translateY(-100%)" }
                            )
                        }
                    >
                        {move || stored_text.get_value()}
                        <div class={css::tooltip_arrow}></div>
                    </div>
                </Portal>
            </Show>
        </div>
    }
}
