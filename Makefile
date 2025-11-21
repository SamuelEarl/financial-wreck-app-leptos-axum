.PHONY: dev build build-preview

# =========================
# DEVELOPMENT
# =========================
leptos-watch:
	cargo leptos watch
	
stylance-watch:
	stylance --watch . --output-dir ./styles/

trigger-leptos-reload:
	cargo run --manifest-path tools/trigger_leptos_reload/Cargo.toml

dev:
	make --jobs=3 leptos-watch stylance-watch trigger-leptos-reload

# =========================
# PRODUCTION BUILD
# =========================
leptos-build:
	cargo leptos build --release -vv

stylance-build:
	stylance . --output-dir ./styles/

build:
	make --jobs=2 leptos-build stylance-build

# =========================
# LOCAL PRODUCTION PREVIEW
# =========================
# After running `make build` I want to be able to test a production build locally with `make preview`.
# However, I don't think I will be able to do that until I have the server-side code ready.
# I might also need to create a script that will copy the server binary (located in `target/server/release`) and the `site` directory (and all files within located in `target/site`) to a `preview` or `dist` directory, export the environment variables, and then run the server binary.
# For details see https://github.com/leptos-rs/start-axum?tab=readme-ov-file#executing-a-server-on-a-remote-machine-without-the-toolchain
# preview:
# 	run preview script


# Kill process on port
# fuser -k 3000/tcp 
