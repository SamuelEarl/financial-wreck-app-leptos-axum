use leptos::prelude::*;

use crate::pages::header_mobile::HeaderMobile;
use crate::pages::header_desktop::HeaderDesktop;

#[derive(Debug, Clone)]
pub struct NavLink {
    pub label: String,
    pub url: String,
}

#[component]
pub fn Header() -> impl IntoView {
    let main_nav: [NavLink; 4] = [
        NavLink { label: String::from("Dashboard"), url: String::from("/dashboard"), },
        NavLink { label: String::from("Net Worth"), url: String::from("/net-worth"), },
        NavLink { label: String::from("Transactions"), url: String::from("/transactions"), },
        NavLink { label: String::from("Budgets"), url: String::from("/budgets"), },
    ];

    view! {
        <HeaderMobile nav=main_nav.clone() />
        <HeaderDesktop nav=main_nav.clone() />
    }
}
