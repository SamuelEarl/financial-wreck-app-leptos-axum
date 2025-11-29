use leptos::prelude::*;
use stylance::*;

import_style!(style, "progress_bar.module.scss");

/// Shows progress toward a goal.
#[component]
pub fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
            class=style::progress_bar
        />
        <br/>
    }
}
