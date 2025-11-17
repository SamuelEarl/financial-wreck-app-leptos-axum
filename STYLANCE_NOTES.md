# Stylance Notes

Look at my StackOverflow post for details on [How to use Stylance with cargo-leptos](https://stackoverflow.com/a/79821297/9453009) for styling a server-side rendered app.

These are some additional notes for this app:

* A common recommendation for production is to use a single, optimized CSS file for the entire app instead of multiple CSS files (e.g. one CSS file per page). 
    * So it is unnecessary to do something like this:
        * Import styles for pages into the head element using the [Stylesheet component](https://docs.rs/leptos_meta/latest/leptos_meta/fn.Stylesheet.html) and...
        * Import styles for components into the components using Stylance's [import_style!()](https://github.com/basro/stylance-rs?tab=readme-ov-file#proc-macro) macro.
* All the styles are concatenated into the `src/assets/styles/main.scss` file.
* I am using separate `src/pages/` and `src/components/` directories.
* The Leptos and Stylance configs inside the `Cargo.toml` file reflect this organization:

```toml
[package.metadata.leptos]
...
style-file = "src/assets/styles/main.scss"


[package.metadata.stylance]
...
output_dir = "./src/assets/styles/"

folders = ["./src/pages/", "./src/components/", "./src/assets/styles/"]
```
