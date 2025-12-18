# Buttons

---

## Variants, Inverted & Widths

`variant`: `Primary`, `Secondary`, `Tertiary`, `Alert`

`inverted`: `true` (default), `false`

* An inverted button will switch the background and foreground colors. NOTE: If you want to create a button with a transparent background then you will have to create a button with custom colors (see below).

`width`: `Auto` (default), `Full`

{{
    <Button
        variant={BtnVariant::Primary}
        inverted={false}
        width={ElementWidths::Full}
        on:click=move |_| { log!("CLICKED"); }
    >
        "Click"
    </Button>
}}

<br />

```rust
use crate::components::buttons::button::Button;
use crate::components::colors_and_sizes::{BtnVariant, ElementWidths};

<Button
    variant={BtnVariant::Primary}
    inverted={false}
    width={ElementWidths::Full}
    on:click=move |_| { log!("CLICKED"); }
>
    "Click"
</Button>
```

<br />

## Colors & Sizes

You can create buttons with custom colors by passing values to the `colors` prop.

* `bg`: background color
* `fg`: foreground color
* `br`: border color
* `ol`: outline color

NOTE: The `colors` prop does not have default values. So if you want to create a button with custom colors, then you will have to pass a color value to each colors prop value.

You can create buttons with custom sizes by passing values to the `sizes` prop.

* `fs`: font size
* `fw`: font weight
* `pv`: padding vertical
* `ph`: padding horizontal

<br />

{{
    <Button
        colors=Some(Colors {
            bg: "var(--dark-red)".to_string(),
            fg: "var(--white)".to_string(),
            br: "var(--dark-red)".to_string(),
            ol: "var(--dark-red)".to_string(),
        })
        sizes=Some(Sizes {
            fs: Some(4),
            fw: Some("normal".to_string()),
            pv: Some(2),
            ph: Some(3),
        })
        on:click=move |_| { log!("CLICKED"); }
    >
        "Click Me"
    </Button>
}}

<br />

```rust
use crate::components::buttons::button::Button;
use crate::components::colors_and_sizes::{Colors, Sizes};

<Button
    colors=Some(Colors {
        bg: "var(--dark-red)".to_string(),
        fg: "var(--white)".to_string(),
        br: "var(--dark-red)".to_string(),
        ol: "var(--dark-red)".to_string(),
    })
    sizes=Some(Sizes {
        fs: Some(4),
        fw: Some("normal".to_string()),
        pv: Some(2),
        ph: Some(3),
    })
    on:click=move |_| { log!("CLICKED"); }
>
    "Click Me"
</Button>
```

NOTE: The `sizes` prop has default values. So if you want to specify values for only some of the size values and use the default values for other size values, then you would specify `..Default::default()`.

{{
    <Button
        variant={BtnVariant::Primary}
        sizes=Some(Sizes {
            pv: Some(4),
            ph: Some(6),
            ..Default::default()
        })
        on:click=move |_| { log!("CLICKED"); }
    >
        "Click Me"
    </Button>
}}

<br />

```rust
use crate::components::buttons::button::Button;
use crate::components::colors_and_sizes::{BtnVariant, Sizes};

<Button
    variant={BtnVariant::Primary}
    sizes=Some(Sizes {
        pv: Some(4),
        ph: Some(6),
        ..Default::default()
    })
    on:click=move |_| { log!("CLICKED"); }
>
    "Click Me"
</Button>
```
