pub mod app;
pub mod pages {
    pub mod layout;
    pub mod auth {
        pub mod sign_in;
    }
    pub mod dashboard {
        pub mod dashboard;
    }
    pub mod goals {
        pub mod goals;
    }
    pub mod bank_accounts {
        pub mod bank_account_layout;
        pub mod transactions;
        pub mod budget;
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
