# Accordions

---

## Standalone Accordions

{{
    <AccordionItem id="standalone1" title="I am independent">
        "I don't need a group wrapper."
    </AccordionItem>
}}

<br />

```rust
<AccordionItem id="standalone1" title="I am independent">
    "I don't need a group wrapper."
</AccordionItem>
```

Each `AccordionItem` requires a unique ID.

<br />

{{
    <AccordionItem id="standalone2" title="Use HTML Elements As Children">
        <div>
          <p>"You can use"</p>
          <p>"any HTML elements"</p>
        </div>
        <div>
          <p>"as the children"</p>
          <p>"of an accordion."</p>
        </div>
    </AccordionItem>
}}

<br />

```rust
<AccordionItem id="standalone2" title="Use HTML Elements As Children">
    <div>
      <p>"You can use"</p>
      <p>"any HTML elements"</p>
    </div>
    <div>
      <p>"as the children"</p>
      <p>"of an accordion."</p>
    </div>
</AccordionItem>
```

<br />

## Accordion Groups

`AccordionGroup` components ensure that only one `AccordionItem` in the group can be open at a time.

{{
    <AccordionGroup>
      <AccordionItem id="1" title="What is 0.8?">
          "Leptos 0.8 is the latest reactive release."
      </AccordionItem>
      <AccordionItem id="2" title="How do Signals work?">
          "They are Copy-able and fine-grained."
      </AccordionItem>
  </AccordionGroup>
}}

<br />

```rust
<AccordionGroup>
    <AccordionItem id="1" title="What is 0.8?">
        "Leptos 0.8 is the latest reactive release."
    </AccordionItem>
    <AccordionItem id="2" title="How do Signals work?">
        "They are Copy-able and fine-grained."
    </AccordionItem>
</AccordionGroup>
```
