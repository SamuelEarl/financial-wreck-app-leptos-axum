# Inputs

---

## Text input

{{
    <div>
        <label>"What is your name?"
            <Input
                attr:placeholder="Type your name"
                attr:value=move || name.get()
                on:input=move |event| {
                    set_name.set(event_target_value(&event));
                }
            />
        </label>
    </div>

    <br/>

    <p>"Hello, " {name} "!"</p>
}}

<br/>

## Email input

{{
    <div>
        <label>"Enter your email"
            <Input
                attr:r#type="email"
                attr:placeholder="Email address"
                attr:value=move || email.get()
                on:input=move |event| {
                    set_email.set(event_target_value(&event));
                }
            />
        </label>
    </div>

    <br/>

    <p>"Email address: " {email}</p>
}}

<br/>

## Password input

{{
    <div>
        <label>"Enter your password"
            <Input
                attr:r#type="password"
                attr:placeholder="Type your password"
                attr:value=move || password.get()
                on:input=move |event| {
                    set_password.set(event_target_value(&event));
                }
            />
        </label>
    </div>

    <br/>

    <p>"Password: " {password}</p>
}}

<br/>

## Number input

{{
    <div>
        <label>"Enter your age"
            <Input
                attr:r#type="number"
                attr:min="0"
                attr:max="100"
                attr:placeholder="Type your age"
                attr:value=move || age.get()
                on:input=move |event| {
                    set_age.set(event_target_value(&event));
                }
            />
        </label>
    </div>

    <br/>

    <p>"Your age: " {age}</p>
}}

<br/>

```rust
use leptos::prelude::*;
use leptos::logging::log;

use crate::components::inputs::input::Input;

#[component]
pub fn PageComponent() -> impl IntoView {
    // Create the signal to hold the value.
    let (name, set_name) = signal("".to_string());  
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (age, set_age) = signal("".to_string());  

    view! {
        // -----------------
        // TEXT INPUT FIELD
        // -----------------
        <div>
            <label>"What is your name?"
                <Input
                    attr:placeholder="Type your name"
                    // READ: Bind the input's value to the signal.
                    // Use attr:value so it gets spread into the <input>.
                    attr:value=move || name.get()
                    // WRITE: Update signal on every keystroke.
                    on:input=move |event| {
                        // `event_target_value` is a helper function built into Leptos. It takes the raw DOM event, finds the element that triggered it, and extracts the current text value safely.
                        set_name.set(event_target_value(&event));
                    }
                />
            </label>
        </div>

        <br/>

        <p>"Hello, " {name} "!"</p>

        <br/>

        // ------------------
        // EMAIL INPUT FIELD
        // ------------------
        <div>
            <label>"Enter your email"
                <Input
                    // In Rust, type is a reserved keyword. To use it as an attribute name in the macro, you must prefix it with r# (the raw identifier prefix).
                    attr:r#type="email"
                    attr:placeholder="Email address"
                    attr:value=move || email.get()
                    on:input=move |event| {
                        set_email.set(event_target_value(&event));
                    }
                />
            </label>
        </div>

        <br/>

        <p>"Email address: " {email}</p>

        <br/>

        // ---------------------
        // PASSWORD INPUT FIELD
        // ---------------------
        <div>
            <label>"Enter your password"
                <Input
                    attr:r#type="password"
                    attr:placeholder="Type your password"
                    attr:value=move || password.get()
                    on:input=move |event| {
                        set_password.set(event_target_value(&event));
                    }
                />
            </label>
        </div>

        <br/>

        <p>"Password: " {password}</p>

        <br/>

        // -------------------
        // NUMBER INPUT FIELD
        // -------------------
        <div>
            <label>"Enter your age"
                <Input
                    attr:r#type="number"
                    attr:min="0"
                    attr:max="100"
                    attr:placeholder="Type your age"
                    attr:value=move || age.get()
                    on:input=move |event| {
                        set_age.set(event_target_value(&event));
                    }
                />
            </label>
        </div>

        <br/>

        <p>"Your age: " {age}</p>
    }
}
```
