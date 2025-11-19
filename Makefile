.PHONY: leptos-watch stylance-watch trigger-leptos-reload dev build

leptos-watch:
	cargo leptos watch
	
stylance-watch:
	stylance --watch . --output-dir ./src/assets/styles/

trigger-leptos-reload:
	cargo run --manifest-path tools/trigger_leptos_reload/Cargo.toml

dev:
	make -j3 leptos-watch stylance-watch trigger-leptos-reload

build:
# 	TODO: What is the build command for Leptos? That needs to go here instead of `cargo leptos`.
	cargo leptos & stylance . --output-dir ./src/assets/styles/


# Kill process on port
# fuser -k 3000/tcp 
