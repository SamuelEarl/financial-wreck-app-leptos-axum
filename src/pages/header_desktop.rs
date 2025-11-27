use leptos::prelude::*;
use leptos_router::components::A;
use stylance::*;

use crate::pages::header::NavLink;

import_style!(style, "header_desktop.module.scss");

#[component]
pub fn HeaderDesktop(nav: [NavLink; 4]) -> impl IntoView {
    view! {
        <header class=style::desktop_header>
            <div>"Header Desktop"</div>
            <ul>
                {
                    nav.into_iter()
                        .map(|link| view! {
                            <li><A href={link.url}>{link.label}</A></li>
                        })
                        .collect_view()
                }
            </ul>
        </header>
    }
}
