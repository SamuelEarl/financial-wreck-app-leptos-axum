use notify::{Event, RecursiveMode, Result, Watcher};
use notify::event::{EventKind, AccessKind};
use notify::event::AccessMode::Write;
use std::{path::Path, path::PathBuf, sync::mpsc};
use std::fs;
use std::time::Duration;
use std::thread;

const WATCH_DIR: &str = "src/assets/styles/stylance/";
const STYLANCE_INDEX: &str = "src/assets/styles/stylance/_index.scss";
const MAIN_SCSS: &str = "src/assets/styles/main.scss";
const TRAILING_COMMENT: &str = "// This comment is added and removed to trigger Leptos hot reloads when SCSS modules are updated.";

fn str_to_event_kind(s: &str) -> EventKind {
    match s.to_lowercase().as_str() {
        // "access" => EventKind::Access(AccessKind::Any),
        // "remove" => EventKind::Remove(RemoveKind::Any),
        // "create" => EventKind::Create(CreateKind::Any),
        // "modify" => EventKind::Modify(ModifyKind::Any),
        // The "close" arm will return the `Access(Close(Write))` enum.
        "close" => EventKind::Access(AccessKind::Close(Write)),
        _ => EventKind::Access(AccessKind::Any),
    }
}

/// THE PROBLEM:
/// It appears that Leptos hot reloading is unable to detect changes in nested Sass/SCSS files that are imported via @use or @import (similar to what is described here for Turborepo https://github.com/vercel/next.js/discussions/82273#discussioncomment-13973065). (Maybe because the file watcher tracks only directly imported files as described above?)
/// This means that changes in partials don’t trigger a hot reload until the main.scss file gets changed directly.

/// THE SOLUTION:
/// This code toggles a comment at the bottom of the main.scss file (i.e. adds or removes the comment) and saves the file (by writing the file back to disk), which triggers a reload.

fn main() -> notify::Result<()> {
    println!("Notify is watching for changes in {}", WATCH_DIR);
    // Create a multi-producer, single-consumer (mpsc) channel that can send values of type Result<Event>.
    // tx = the sender - you can send messages into the channel through the sender.
    // rx = the receiver - you can read events from the channel through the receiver.
    // notify delivers file events on a background thread, so it must send them back to your main thread. Using a channel is the standard way to do that.
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    // This creates a filesystem watcher that automatically sends file events into your channel, using the best backend for your operating system, and returns early if watcher creation fails.
    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new(WATCH_DIR), RecursiveMode::Recursive)?;

    for res in rx {
        // println!("RES: {:?}", res);
        match res {
            Ok(event) => {
                println!("File Changed: {:?}", event.paths);
                let path = &event.paths[0];
                // Convert STYLANCE_INDEX to a PathBuf data type.
                let stylance_index_path = &PathBuf::from(STYLANCE_INDEX);                
                // Whenever any SCSS module in this project gets updated, each module in the the stylance/ directory goes through a process of events like this (I think in this order): Access, Remove, Create, Modify, Close. (You can see this if you uncomment the println!("RES: {:?}", res); statement above.)
                // The problem is that whenever _any_ SCSS module in this project gets updated, for some reason this file watcher also sends an event for the main.scss file, indicating that it was also changed. Each time that main.scss event comes through, the toggle_trailing_comment() function gets called, which changes the main.scss file and creates another event for the main.scss file. This results in an infinite loop where the main.scss file is constantly being updated.
                // So this `if` statement checks if the Stylance _index.scss file has been changed AND if the event.kind equals the `Access(Close(Write))` enum. This results in the toggle_trailing_comment() being called only once each time a SCSS file is updated and saved, which prevents the infinite loop.
                if path.ends_with(stylance_index_path) && event.kind == str_to_event_kind("close") {
                    // Debounce a bit to let writes settle:
                    thread::sleep(Duration::from_millis(250));
                    if let Err(e) = toggle_trailing_comment() {
                        eprintln!("Error updating main.scss: {:?}", e);
                    }
                }
            }
            Err(e) => eprintln!("Watch Error: {:?}", e),
        }
    }

    Ok(())
}

fn toggle_trailing_comment() -> std::io::Result<()> {
    let path = Path::new(MAIN_SCSS);

    let content = fs::read_to_string(path)?;
    let trimmed = content.trim_end();

    // detect if last non-empty line is the comment
    let mut lines: Vec<&str> = trimmed.split('\n').collect();
    let last_line = lines.last().unwrap_or(&"");

    if *last_line == TRAILING_COMMENT {
        // remove it
        println!("Removing trailing comment from main.scss");
        lines.pop();
    } else {
        // add it
        println!("Adding trailing comment to main.scss");
        lines.push(TRAILING_COMMENT);
    }

    // Rejoin the lines and ensure file ends with a single newline
    let new_content = format!("{}\n", lines.join("\n"));

    fs::write(path, new_content)?;
    Ok(())
}
