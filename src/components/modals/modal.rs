use leptos::prelude::*;
use leptos::html::{Div, HtmlElement}; // Typed HTML elements
// use web_sys::window;
use leptos::ev::MouseEvent;
use leptos_use::use_window;
use stylance::*;

import_style!(css, "modal.module.scss");

#[component]
pub fn Modal(
    #[prop(into, default = "".to_string())] 
    title: String,
    
    #[prop(default = true)] 
    scrolling_body: bool,
    
    #[prop(into, default = "".to_string())] 
    focus_element: String,
    
    #[prop(default = false)] 
    disabled: bool,
    
    #[prop(default = true)] 
    show_close_btn: bool,
    
    #[prop(into, default = "auto".to_string())] 
    modal_width: String,
    
    #[prop(into, default = "var(--white)".to_string())] 
    modal_background_color: String,
    
    #[prop(into)]
    on_close_modal_callback: Callback<MouseEvent>,

    children: Children,
    
    #[prop(optional)]
    modal_footer_left: Option<Children>,
    
    #[prop(optional)]
    modal_footer_right: Option<Children>,
) -> impl IntoView {
    // Refs for DOM access
    let modal_content_container_ref = NodeRef::<Div>::new();
    let modal_body_wrapper_ref = NodeRef::<Div>::new();

    // Logic to set scrolling body
    let set_scrolling_body = move || {
        // 2. Use the hook
        let window_obj = use_window();

        // 3. Unwrap references
        // window_obj.as_ref() gives us Option<&web_sys::Window>
        if let (Some(win), Some(content_el), Some(body_el)) = (
            window_obj.as_ref(), 
            modal_content_container_ref.get(),
            modal_body_wrapper_ref.get(),
        ) {
            let window_height = win
                .inner_height()
                .ok()
                .and_then(|h| h.as_f64())
                .unwrap_or(0.0);

            let content_height = content_el.get_bounding_client_rect().height();
            let body_height = body_el.get_bounding_client_rect().height();

            let modal_top_and_bottom_heights = content_height - body_height;
            let new_height = window_height - modal_top_and_bottom_heights;

            // Apply styles using web_sys logic
            // let _ = body_el.style().set_property("height", &format!("{}px", new_height));
            let _ = web_sys::HtmlElement::style(&body_el).set_property("height", &format!("{}px", new_height));
            let _ = body_el.style().set_property("overflow", "auto");
        }
    };

    // Logic to set focus
    let set_focus = move || {
        if !focus_element.is_empty() {
            let window_obj = use_window();
            
            // 4. Chain access: Window -> Document -> Element
            if let Some(win) = window_obj.as_ref() {
                if let Some(document) = win.document() {
                    if let Some(el) = document.get_element_by_id(&focus_element) {
                        if let Ok(html_el) = el.dyn_into::<HtmlElement>() {
                            let _ = html_el.focus();
                        }
                    }
                }
            }
        }
    };

    // 5. Run effects
    Effect::new(move |_| {
        if scrolling_body {
            set_scrolling_body();
        }
        set_focus();
    });

    // Helper for border radius CSS
    let body_border_radius = move || {
        let has_title = !title.is_empty();
        let has_footer = modal_footer_left.is_some() || modal_footer_right.is_some();

        if !has_title && !has_footer {
            "border-radius: var(--radius);"
        } else if !has_title {
            "border-radius: var(--radius) var(--radius) 0 0;"
        } else if !has_footer {
            "border-radius: 0 0 var(--radius) var(--radius);"
        } else {
            ""
        }
    };

    view! {
        <Show when=move || show_close_btn>
            <div id="close-btn-container">
                <button
                    id="close"
                    disabled=disabled
                    on:click=on_close_modal_callback
                    style="background: transparent; border: none; color: var(--white); cursor: pointer; padding: 8px;"
                >
                    <span style="font-size: 24px;">"✕"</span> 
                </button>
            </div>
        </Show>

        <div id="fp-modal">
            <div 
                id="modal-content-container" 
                class="fp-animatetop"
                node_ref=modal_content_container_ref
            >
                <div 
                    id="modal-content" 
                    style:width=modal_width
                    style:background-color=modal_background_color
                >
                    <Show when=move || !title.is_empty()>
                        <header id="modal-header">
                            <h3 id="modal-title" tabindex="0">
                                {title.clone()}
                            </h3>
                        </header>
                    </Show>

                    <div 
                        id="modal-body-wrapper" 
                        style=body_border_radius
                        node_ref=modal_body_wrapper_ref
                    >
                        {children()}
                    </div>

                    <Show when=move || modal_footer_left.is_some() || modal_footer_right.is_some()>
                        <footer id="modal-footer">
                            <div id="modal-footer-left">
                                {modal_footer_left.map(|f| f())}
                            </div>
                            <div id="modal-footer-right">
                                {modal_footer_right.map(|f| f())}
                            </div>
                        </footer>
                    </Show>
                </div>
            </div>
        </div>
    }
}
