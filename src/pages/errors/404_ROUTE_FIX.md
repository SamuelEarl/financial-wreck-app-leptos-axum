# The catch-all route is causing my CSS to not work

This is a classic Single Page Application (SPA) / SSR issue.

The Cause: Your browser is requesting your CSS file (e.g., /pkg/my-app.css), but because that file path doesn't match a defined route like / or /about, the *any wildcard is catching it.

Instead of returning the CSS code, your server is returning the HTML of your 404 page. When the browser tries to parse that HTML as CSS, it fails, and your styles break.

Here is how to fix it, depending on whether the issue is Server Configuration (most likely) or HTML Paths.

## Fix 1: Configure Static Files on the Server (Most Likely)
You need to tell your server (Axum or Actix) to serve the static files (in the /pkg or /assets folder) before it tries to match the Leptos wildcard route.

### If you are using Axum (main.rs)
In your app router configuration, you likely have a .fallback(leptos_routes_handler). You must ensure nest_service for your static files is attached before or alongside the fallback so strictly defined files take precedence.

```rust
// In your main function where you define 'app':
let app = Router::new()
    .leptos_routes(&state, routes, App)
    // 1. VITAL: Serve the pkg folder explicitly
    // This catches /pkg/style.css BEFORE it hits the *any fallback
    .nest_service("/pkg", ServeDir::new("target/site/pkg"))
    .nest_service("/assets", ServeDir::new("assets")) // If you have an assets folder
    // 2. The fallback handles everything else (including your *any route)
    .fallback(file_and_error_handler)
    .with_state(state);
```
