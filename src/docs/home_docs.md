# Docs Home

One of the features of this component library is that it uses outlines extensively to display hover and focus states as well as substantial color contrasts. Both of these design ideas help to improve a11y. For those who are colorblind or are visually impaired, the color transitions and color contrasts that are used in many UI libraries are difficult to see. This component library uses outlines instead of color transitions for hover and focus states and stark color constrasts as much as possible.

In addition, the colors and fonts are fully customizable to create a custom theme relatively easily.

---

<!-- <div>
  <ul>
    <li><Link href="/docs/components/ui">UI Components</Link></li>
    <li><Link href="/docs/components/data-viz">Data Viz Components</Link></li>
    <li><Link href="/docs/utility-classes">Utility Classes</Link></li>
  </ul>
</div>

--- -->

## General Component Props

<h3 id="colors">colors</h3>

The components all use default colors for background color, foreground color, border color, and outline color, which can all be changed in the `styles/_theme.scss` file and/or the `src/components/colors_and_sizes.rs` file. Many of the components also have a `colors` prop that can be used for customizing the colors of the component.

Note that this design system uses background (`bg`), foreground (`fg`), border (`br`), and outline (`ol`) colors extensively for default colors and for custom colors. For most components the background propery is used for the background color, the foreground property is used for the text and icon colors, the border color is used to display clearly defined borders, and the outline color is used to show an outline on hover and focus states, which helps with a11y and keyboard navigation.

For the components that accept a `colors` prop, you can pass an object that uses the `Colors` struct, like this:

```rust
use crate::components::buttons::button::Button;
use crate::components::colors_and_sizes::{BtnVariant, Colors, Sizes};

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

// The `sizes` prop has default values. So if you want to specify 
// values for only some of the size values and use the default values
// for other size values, then you would specify `..Default::default()`.
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

Each property (`bg`, `fg`, `br`, `ol`) can take any CSS color value.

<br/>

**For components other than `<Button>` and `<Link btnStyles>` components**

The default color values are defined in the `styles/_theme.scss` file with the following variables:

```css
--element-bg
--element-fg
--element-br
--element-ol
```

These default values are set in the `src/components/colors_and_sizes.rs` file. These default values can be changed by changing the corresponding CSS variables in the `styles/_theme.scss` file.

Each color property is optional, so you can specify only the colors you want to change and the other colors will use default values.

<br/>

**For `<Button>` and `<Link btnStyles>` components**

The default values are the `primary` variant colors (i.e. `var(--primary-bg)`, `var(--primary-fg)`) and can be changed in the `styles/_theme.scss` file. (See the docs for the `<Button>` and `<Link btnStyles>` components for more details.)

If you want to change any colors then you will have to specify **all** of the properties in the `colors` prop. If you do not, then you will get unexpected colors for your `<Button>` and `<Link btnStyles>` components.

The `colors` prop will override the other props that deal with colors (i.e. `variant`, `inverted`).

<br/>
<br/>

<h3 id="sizes">sizes</h3>

The components all use default values for font size, font weight, and padding (vertical and horizontal), which can all be changed in the `styles/_theme.scss` file and/or the `src/components/colors_and_sizes.rs` file. Many of the components also have a `sizes` prop that can be used for customizing the sizes of the component.

Refer to the `<Button>` example above to see how to use the `Sizes` struct to customize the sizes of components.

These are the default size values:

```rust
{
    fs: Some(4),
    fw: Some("normal".to_string()),
    pv: Some(2),
    ph: Some(3),
}
```

The size properties `fs`, `pv`, and `ph` reference the `--size-{n}` CSS variables that are define in the `styles/_theme.scss` file.

So the possible values for `fs`, `pv`, and `ph` are 0 to 25 (or whatever the maximum number is that you have defined for the `--size-{n}` CSS variable in the `styles/_theme.scss` file).

`fw` can be any font weight.

Each size property is optional, so you can specify only the sizes you want to change and the other sizes will use default values.

The CSS size variables (`--size-{n}`) in the `styles/_theme.scss` file use a 16px base value (i.e. 1rem = 16px). Since each size number increments by .25rem (or 4px), you can determine the pixel size for each number that is passed to the `sizes` prop by multiplying the number by 4. (e.g. `fs: Some(6)` = 6 x 4px = 24px font size).

<br/>

### Important note about how the `colors` and `sizes` props are used in components

This note describes how the `colors` and `sizes` props are used in components to style individual elements within a component. Some components only have a single element to style (e.g. buttons, input fields) while others have multiple elements that need to be styled in different ways (e.g. accordions, select boxes).

For most elements within a component, you can place something like this on the element:

```
style=format!("{} {}", get_element_colors(colors, false).all, get_element_sizes(sizes, false).all)
```

…and the `colors` and `sizes` that are passed to the component will be applied to the element. If any color or size properties are not passed to the component, then the default values will be used in place of those colors and/or sizes.

However, if you only want to apply one or two color or size styles to an element (within a component), then you can specify the style that you want to apply to the element like this:

```
style=format!("background-color: {}; font-size: {};", get_element_colors(colors, false).bg, get_element_sizes(sizes, false).fs)
```

**_NOTE: I have not implemented and used the `get_element_colors()` function yet, so the actual API might be a little different than the above examples._**

<br/>

### Rest props

Rest props are used to capture the rest of the properties that are not specifically defined in the component. For example, you could pass an `id="abc"` prop to a `<Button>` component and the `id` will show up in the `<button>` in the DOM.

These components use an `attributes` prop to handle any rest props. See the `<Button>` component for details.

```rust
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
```
