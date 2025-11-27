use leptos::prelude::*;
use leptos_router::components::A;
use stylance::*;

use crate::pages::header::NavLink;

import_style!(style, "header_mobile.module.scss");

#[component]
pub fn HeaderMobile(nav: [NavLink; 4]) -> impl IntoView {
    view! {
        <div>"Header Mobile"</div>
        <header class=style::mobile_header>
            <div class=style::content>
                <div class="open_menu_btn_container">
                    <button>"OPEN"</button>
                </div>
            </div>
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
