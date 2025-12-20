use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos::logging::{log, error};
use leptos::task::spawn_local;
use stylance::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::components::{
    colors_and_sizes::{BtnVariant, Sizes},
    buttons::button::Button,
};
use crate::utils::{
    format_currency::format_currency,
    dates::get_utc_iso_date,
};
use crate::pages::net_worth::net_worth_article::NetWorthArticle;

import_style!(css, "net_worth.module.scss");

#[component]
pub fn NetWorth() -> impl IntoView {
    provide_meta_context();

    let (net_worth, set_net_worth) = signal(0);
    set_net_worth.set(1_000_000);

    view! {
        <Title text="Financial Wreck | Net Worth"/>

        <div class={css::h1_wrapper}>
            <h1 class={css::h1}>"Net Worth"</h1>
            <NetWorthArticle />
        </div>

        <h2>{move || format_currency(net_worth.get(), None)}</h2>

        <br />

        <p>"Add financial accounts, transfer money between accounts, and add bill pay alerts"</p>

        // <div class={css::al_wrapper}>
        //     <h2 class={css::h2}>"Assets"</h2>
        //     <div class={css::btns_container}>
        //         <Button
        //             variant={BtnVariant::Primary}
        //             sizes=Some(Sizes {
        //                 pv: Some(0),
        //                 ph: Some(2),
        //                 ..Default::default()
        //             })
        //             on:click=move |_| { 
        //                 spawn_local(async {
        //                     let new_asset = create_asset().await;

        //                 });
        //             }
        //         >
        //             "Add"
        //         </Button>
        //     </div>
        // </div>
        <AssetList />
        
        <br />

        <div class={css::al_wrapper}>
            <h2 class={css::h2}>"Liabilities"</h2>
            <div class={css::btns_container}>
                <Button
                    variant={BtnVariant::Primary}
                    sizes=Some(Sizes {
                        pv: Some(0),
                        ph: Some(2),
                        ..Default::default()
                    })
                    on:click=move |_| { log!("ADD"); }
                >
                    "Add"
                </Button>
            </div>
        </div>

    }
}

#[derive(Debug)]
pub struct NetWorthCategory {
    pub category: &'static str,
    pub label: &'static str,
}

static ASSET_CATEGORIES: [NetWorthCategory; 16] = [
    NetWorthCategory { category: "bank_account", label: "Bank Account (checking, savings)", },
    NetWorthCategory { category: "retirement_investment", label: "Retirement Investment (401k, IRA)", },
    NetWorthCategory { category: "non_retirement_investment", label: "Non-Retirement Investment (mutual funds, stocks, bonds)", },
    NetWorthCategory { category: "cod", label: "Certificate of Deposit", },
    NetWorthCategory { category: "cash_value", label: "Cash Value of Life Insurance", },
    NetWorthCategory { category: "annuity", label: "Annuity", },
    NetWorthCategory { category: "pension", label: "Pension", },
    NetWorthCategory { category: "hsa", label: "Health Savings Account (HSA)", },
    NetWorthCategory { category: "cryptocurrency", label: "Cryptocurrency", },
    NetWorthCategory { category: "cash", label: "Cash On-Hand", },
    NetWorthCategory { category: "real_estate", label: "Real Estate", },
    NetWorthCategory { category: "vehicle", label: "Vehicle", },
    NetWorthCategory { category: "personal_item", label: "Personal Item", },
    NetWorthCategory { category: "business", label: "A Business (your portion only)", },
    NetWorthCategory { category: "money_owed_to_you", label: "Money Owed To You", },
    NetWorthCategory { category: "other_asset", label: "Other Asset", },
];

static LIABILITY_CATEGORIES: [NetWorthCategory; 3] = [
    NetWorthCategory { category: "loan", label: "Loan (mortgage, car, education, etc)", },
    NetWorthCategory { category: "credit_card", label: "Credit Card", },
    NetWorthCategory { category: "other_liability", label: "Other Liability", },
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Asset {
  uuid: Uuid,
  category: String,
//   subcategory: String,
  name: String,
  value: i64,
  login_url: String,
  sort_order: u64,
  created_at: String,
  calculated_bank_balance: bool,
}

#[component]
pub fn AssetList() -> impl IntoView {
    // 1. Create the Resource
    // The first argument `|| ()` is the "dependency." 
    // Since it's empty, this resource runs exactly once (on load).
    let assets_resource = Resource::new(|| (), |_| get_assets());

    view! {
        <div class={css::al_wrapper}>
            <h2 class={css::h2}>"Assets"</h2>
            <div class={css::btns_container}>
                <Button
                    variant={BtnVariant::Primary}
                    sizes=Some(Sizes {
                        pv: Some(0),
                        ph: Some(2),
                        ..Default::default()
                    })
                    on:click=move |_| { 
                        spawn_local(async move {
                            // 1. Call the server function
                            // We match on the Result to ensure it succeeded
                            match create_asset().await {
                                Ok(new_asset) => {
                                    // 2. LOCALLY update the resource
                                    // We don't need to refetch the whole list!
                                    assets_resource.update(|current_state| {
                                        // current_state is &mut Option<Result<Vec<Asset>, Error>>
                                        // We only want to push if we currently have a valid list
                                        if let Some(Ok(list)) = current_state {
                                            list.push(new_asset);
                                        }
                                    });
                                },
                                Err(e) => error!("Failed to create asset: {}", e),
                            }
                        });
                    }
                >
                    "Add"
                </Button>
            </div>
        </div>

        // 2. Wrap in Suspense to handle the "Loading..." state
        <Suspense fallback=move || view! { <p>"Loading assets..."</p> }>
            
            // 3. Read the resource
            // We use ErrorBoundary to handle if the server function fails (Result::Err)
            <ErrorBoundary fallback=|_| view! { <p>"Something went wrong."</p> }>
                {move || {
                    // .get() returns Option<Result<Vec<Asset>, Error>>
                    // .map handles the "Loaded" state
                    assets_resource.get().map(|result| {
                        match result {
                            Ok(assets) => view! {
                                <div class="asset-grid">
                                    // 4. Iterate over the Vec<Asset>
                                    // <For> is efficient for lists that might change
                                    // Note: We use assets.clone() here because the resource
                                    // owns the data. This is standard for small lists.
                                    <For
                                        each=move || assets.clone()
                                        key=|asset| asset.uuid // Use UUID as the unique key
                                        children=|asset| view! {
                                            <div class="asset-card">
                                                <h4>{asset.name}</h4>
                                                <p>"Value: $" {asset.value}</p>
                                                <p>"Category: " {asset.category}</p>
                                            </div>
                                        }
                                    />
                                </div>
                            }.into_any(),
                            Err(e) => view! { <p>"Error loading data: " {e.to_string()}</p> }.into_any(),
                        }
                    })
                }}
            </ErrorBoundary>
        </Suspense>
    }
}

#[server(name = CreateAsset, prefix = "/api/create-asset")]
pub async fn create_asset() -> Result<Asset, ServerFnError> {
    // Define the data as requested
    let asset = Asset { 
        // uuid: Some(Uuid::now_v7().to_string()),
        uuid: Uuid::now_v7(),
        // TODO: I probably need to turn the categories array into an enum.
        category: String::from("Bank Account"),
        name: String::from("USAA Savings"),
        value: 1_000_000, // $10,000.00
        login_url: String::from("https://usaa.com"),
        sort_order: 0,
        created_at: get_utc_iso_date(),
        calculated_bank_balance: true,
    };

    // TODO: Save the new asset to the database here...

    Ok(asset)
}

#[server(name = GetAssets, prefix = "/api/get-assets")]
pub async fn get_assets() -> Result<Vec<Asset>, ServerFnError> {
    // Define the data as requested
    let assets = vec![
        Asset { 
            // uuid: Some(Uuid::now_v7().to_string()),
            uuid: Uuid::now_v7(),
            // TODO: I probably need to turn the categories array into an enum.
            category: String::from("Bank Account"),
            name: String::from("USAA Savings"),
            value: 1_000_000, // $10,000.00
            login_url: String::from("https://usaa.com"),
            sort_order: 0,
            created_at: get_utc_iso_date(),
            calculated_bank_balance: true,
        },
        Asset { 
            // uuid: Some(Uuid::now_v7().to_string()),
            uuid: Uuid::now_v7(),
            // TODO: I probably need to turn the categories array into an enum.
            category: String::from("IRA"),
            name: String::from("Vangard"),
            value: 10_000_000, // $10,000.00
            login_url: String::from("https://vangard.com"),
            sort_order: 1,
            created_at: get_utc_iso_date(),
            calculated_bank_balance: false,
        },
    ];

    Ok(assets)
}
