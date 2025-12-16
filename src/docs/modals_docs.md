# Modals

---

{{
    let (show_modal, set_show_modal) = RwSignal::new(false).split();

    <button on:click=move |_| set_show_modal.set(true)>"Open Modal"</button>

    <Show when=move || show_modal.get()>
        <Modal
            title="My Leptos Modal"
            on_close_modal_callback=move |_| set_show_modal.set(false)
            // Use a slot for the left footer
            modal_footer_left=Box::new(move || view! {
                <button>"Cancel"</button>
            })
            // Use a slot for the right footer
            modal_footer_right=Box::new(move || view! {
                <button>"Confirm"</button>
            })
        >
            // Main Body Content
            <p>"This is the body of the modal!"</p>
            <p>"It will scroll if the content is too long."</p>
        </Modal>
    </Show>
}}

<br />

```rust
#[component]
pub fn ParentPage() -> impl IntoView {
    let (show_modal, set_show_modal) = RwSignal::new(false).split();

    view! {
        <button on:click=move |_| set_show_modal.set(true)>"Open Modal"</button>

        <Show when=move || show_modal.get()>
            <Modal
                title="My Leptos Modal"
                on_close_modal_callback=move |_| set_show_modal.set(false)
                // Use a slot for the left footer
                modal_footer_left=Box::new(move || view! {
                    <button>"Cancel"</button>
                })
                // Use a slot for the right footer
                modal_footer_right=Box::new(move || view! {
                    <button>"Confirm"</button>
                })
            >
                // Main Body Content
                <p>"This is the body of the modal!"</p>
                <p>"It will scroll if the content is too long."</p>
            </Modal>
        </Show>
    }
}
```
