pub mod app;
pub mod pages {
    pub mod layout;
    pub mod header;
    pub mod header_mobile;
    pub mod header_desktop;
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
pub mod components {
    pub mod buttons {
        pub mod button;
    }
    pub mod icons {
        pub mod icon;
    }
    pub mod progress_bars {
        pub mod progress_bar;
    }
    pub mod colors_and_sizes;
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
