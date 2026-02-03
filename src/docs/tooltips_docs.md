# Tooltips

---

{{
    <Tooltip text="This is the tooltip text">
        <span class="tooltip-info-icon">"?"</span>
    </Tooltip>
}}

<br/>

{{
    <Tooltip text="You can have multi-line content
    
        by adding line breaks in the string.">
        <span>"Multi-line content"</span>
    </Tooltip>
}}

<br/>

```rust
use leptos::prelude::*;

use crate::components::tooltips::tooltip::Tooltip;

#[component]
pub fn PageComponent() -> impl IntoView {
    view! {
        
    }
}
```
