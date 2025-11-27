use leptos::prelude::*;
use leptos_router::components::Outlet;
use stylance::*;

use crate::pages::header_mobile::HeaderMobile;
use crate::pages::header_desktop::HeaderDesktop;

struct Link {
    label: &str,
    url: &str,
}

#[component]
pub fn Header() -> impl IntoView {
    let main_nav: [Link; 4] = [
        Link { label: "Dashboard", url: "/dashboard" },
        Link { label: "Net Worth", url: "/net-worth" },
        Link { label: "Bank Accounts", url: "/bank-accounts" },
        Link { label: "Budgets", url: "/budgets" },
    ];

    view! {
        <HeaderMobile />
        <HeaderDesktop />
    }
}
