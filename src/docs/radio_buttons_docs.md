<!-- TODO: I need ask Gemini how to loop over a vector and display one <RadioButton> component for each item in the vector. -->

```rust
{{
    <label>"Select your favorite language"
        <RadioGroup 
            name="languages" 
            value=favorite
            set_value=set_favorite
        >
            <RadioButton value="rust" label="Rust" />
            <RadioButton value="ts" label="TypeScript" />
            <RadioButton value="python" label="Python" />
        </RadioGroup>
    </label>

    <br/>

    <p>"Currently selected: " {move || favorite.get()}</p>
}}
```

<br/>

```rust
{{
    <label>"Select your favorite language"
        <RadioButtons 
            group_name="languages"
            options=programming_languages
            default_value="mojo"
            value=favorite
            set_value=set_favorite
        />
    </label>

    <br/>

    <p>"Currently selected: " {move || favorite.get()}</p>
}}
```

```rust
<label>"Select your favorite language"
    <RadioButtons 
        group_name="languages"
        options=programming_languages
        default_value="mojo"
        value=favorite // Pass the signal getter to the `value` prop.
        set_value=set_favorite // Pass the signal setter to the `set_value` prop.
    />
</label>

<br/>

<p>"Currently selected: " {move || favorite.get()}</p>
```
