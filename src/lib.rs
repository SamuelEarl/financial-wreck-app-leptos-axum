pub mod app;
pub mod client_router;
pub mod nav_links;

pub mod components {
    pub mod accordions {
        pub mod accordion;
    }
    pub mod buttons {
        pub mod button;
    }
    pub mod colors_and_sizes;
    pub mod dialogs {
        pub mod dialog;
    }
    pub mod icons {
        pub mod icon;
    }
    pub mod layouts {
        pub mod layout;
        pub mod mobile_header;
        pub mod sidebar;
    }
    pub mod progress_bars {
        pub mod progress_bar;
    }
    pub mod selects {
        pub mod select;
    }
}

// Only compile this module if the feature is active.
#[cfg(feature = "docs")]
pub mod docs {
    pub mod home_docs;
    pub mod accordions_docs;
    pub mod buttons_docs;
    pub mod dialogs_docs;
}

pub mod pages {
    pub mod auth {
        pub mod sign_in;
    }
    pub mod bank_accounts {
        pub mod bank_transactions_and_budgets_list;
        pub mod budget;
        pub mod transactions;
        pub mod transactions_budget_wrapper;
    }
    pub mod dashboard {
        pub mod dashboard;
    }
    pub mod errors {
        pub mod not_found;
    }
    pub mod goals {
        pub mod goals;
    }
    pub mod net_worth {
        pub mod net_worth;
        pub mod net_worth_article;
        pub mod net_worth_models;
    }
}

pub mod routes {
    pub mod docs_routes;
    pub mod pages_routes;
}

pub mod utils {
    pub mod format_currency;
    pub mod dates;
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
