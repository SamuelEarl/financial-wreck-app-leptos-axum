run-dev:
	cargo leptos watch & stylance --watch . --output-dir ./src/assets/styles/

run-build:
# 	TODO: What is the build command for Leptos? That needs to go here instead of `cargo leptos`.
	& cargo leptos & stylance . --output-dir ./src/assets/styles/
