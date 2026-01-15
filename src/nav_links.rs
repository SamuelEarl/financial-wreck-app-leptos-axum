#[derive(Debug)]
pub struct NavLink {
    pub text: &'static str,
    pub url: &'static str,
}

pub static MAIN_NAV: [NavLink; 4] = [
    NavLink { text: "Dashboard", url: "/dashboard", },
    NavLink { text: "Goals", url: "/goals", },
    NavLink { text: "Net Worth & Accounts", url: "/net-worth", },
    NavLink { text: "Transactions & Budgets", url: "/bank-transactions-and-budgets-list", },
];

#[cfg(feature = "docs")]
pub static DOCS_NAV: [NavLink; 6] = [
    NavLink { text: "Docs Home", url: "/docs", },
    NavLink { text: "Accordions", url: "/docs/accordions", },
    NavLink { text: "Buttons", url: "/docs/buttons", },
    NavLink { text: "Dialogs", url: "/docs/dialogs", },
    NavLink { text: "Radio Buttons", url: "/docs/radio-buttons", },
    NavLink { text: "Select", url: "/docs/select", },
];
