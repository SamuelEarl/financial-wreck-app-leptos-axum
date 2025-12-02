use leptos::prelude::*;

use crate::pages::mobile_header::MobileHeader;
use crate::pages::header_desktop::HeaderDesktop;

#[derive(Debug, Clone)]
pub struct NavLink {
    pub text: String,
    pub url: String,
}

#[component]
pub fn Header() -> impl IntoView {
    let main_nav: [NavLink; 4] = [
        NavLink { text: String::from("Dashboard"), url: String::from("/dashboard"), },
        NavLink { text: String::from("Net Worth"), url: String::from("/net-worth"), },
        NavLink { text: String::from("Transactions"), url: String::from("/transactions"), },
        NavLink { text: String::from("Budgets"), url: String::from("/budgets"), },
    ];

    view! {
        <MobileHeader nav=main_nav.clone() />
        <HeaderDesktop nav=main_nav.clone() />
    }
}
