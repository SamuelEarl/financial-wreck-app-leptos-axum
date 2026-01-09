# The .DEFAULT_GOAL defines which target is run when no target is specified. In the following example, the default is the build target.
# .DEFAULT_GOAL := build

# The .PHONY line keeps `make` from getting confused if a directory or file in your project has the same name as one of the listed targets.
.PHONY: kill dev build build-preview generate-css-icons

# Each possible operation is called a target and the following definitions are the target definitions. 
# The word before the colon (:) is the name of the target. 
# Any words after the target (like `kill` in the line `dev: kill`) are the other targets that must be run before the specified target runs. 
# The tasks that are performed by the target are on the indented lines after the target.
# =========================
# DEVELOPMENT
# =========================
leptos-watch:
	APP_ENV=development cargo leptos watch --features docs
	
stylance-watch:
	stylance --watch . --output-dir ./styles/

trigger-leptos-reload:
	cargo run --manifest-path tools/trigger_leptos_reload/Cargo.toml

# Kill any orphan processes.
# The `-` at the start and `|| true` at the end prevent the Makefile from stopping if there are no processes to kill (which would normally throw an error).
kill:
	-kill -9 $(lsof -t -i:3000) || true
	-kill -9 $(lsof -t -i:3001) || true

dev: kill
	make --jobs=3 leptos-watch stylance-watch trigger-leptos-reload

# ----------------------------------------------------------------
# Node.js processes for working with CSS icons during development
# ----------------------------------------------------------------
install-node-packages:
	cd generate-css-icons && npm install

generate-css-icons:
	cd generate-css-icons && node index.js


# =========================
# PRODUCTION BUILD
# =========================
# The --release flag creates an optimized, production-ready binary version.
# The -vv flag means to use very verbose output.
leptos-build:
	APP_ENV=production cargo leptos build --release -vv

stylance-build:
	stylance . --output-dir ./styles/

build:
	make --jobs=2 leptos-build stylance-build

# =========================
# LOCAL PRODUCTION PREVIEW
# =========================
# After running `make build` I want to be able to test a production build locally with `make preview`.
# However, I don't think I will be able to do that until I have the server-side code ready.
# I might also need to create a script that will copy the server binary (located in `target/server/release`) and the `site` directory (and all files located within `target/site`) to a `preview` or `dist` directory, export the environment variables, and then run the server binary.
# For details see https://github.com/leptos-rs/start-axum?tab=readme-ov-file#executing-a-server-on-a-remote-machine-without-the-toolchain
# preview:
# 	run preview script


# Kill process on port
# fuser -k 3000/tcp 
