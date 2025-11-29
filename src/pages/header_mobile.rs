use leptos::prelude::*;
use leptos_router::components::A;
use stylance::*;

use crate::pages::header::NavLink;
use crate::components::icons::icon::Icon;

import_style!(css, "header_mobile.module.scss");

#[component]
pub fn HeaderMobile(nav: [NavLink; 4]) -> impl IntoView {
    view! {
        <header class={css::mobile_header}>
            <div class={css::content}>
                <div class="open_menu_btn_container">
                    <button>
                        <Icon
                            icon="material-symbols:menu".to_string()
                            attr:style="font-size: var(--size-8)"
                        />
                    </button>
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
