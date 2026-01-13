use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos::logging::{log, error};
use leptos::task::spawn_local;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use stylance::*;

use crate::components::{
    colors_and_sizes::{BtnVariant, Colors, Sizes, ElementWidths},
    buttons::button::Button,
    selects::select::{Select, SelectText, SelectOptions, OptionData},
};
use crate::utils::{
    format_currency::format_currency,
    dates::get_utc_iso_date,
};
use crate::pages::net_worth::{
    net_worth_article::NetWorthArticle,
    // net_worth_models::AssetCategory,
};

import_style!(css, "net_worth.module.scss");

#[component]
pub fn NetWorth() -> impl IntoView {
    provide_meta_context();

    let asset_options = vec![
        OptionData { group: None, value: "bank_account".to_string(), label: "Bank Account (checking, savings)".to_string(), },
        OptionData { group: None, value: "retirement_investment".to_string(), label: "Retirement Investment (401k, IRA)".to_string(), },
        OptionData { group: None, value: "non_retirement_investment".to_string(), label: "Non-Retirement Investment (mutual funds, stocks, bonds)".to_string(), },
        OptionData { group: None, value: "cod".to_string(), label: "Certificate of Deposit".to_string(), },
        OptionData { group: None, value: "cash_value".to_string(), label: "Cash Value of Life Insurance".to_string(), },
        OptionData { group: None, value: "annuity".to_string(), label: "Annuity".to_string(), },
        OptionData { group: None, value: "pension".to_string(), label: "Pension".to_string(), },
        OptionData { group: None, value: "hsa".to_string(), label: "Health Savings Account (HSA)".to_string(), },
        OptionData { group: None, value: "cryptocurrency".to_string(), label: "Cryptocurrency".to_string(), },
        OptionData { group: None, value: "cash".to_string(), label: "Cash On-Hand".to_string(), },
        OptionData { group: None, value: "real_estate".to_string(), label: "Real Estate".to_string(), },
        OptionData { group: None, value: "vehicle".to_string(), label: "Vehicle".to_string(), },
        OptionData { group: None, value: "personal_item".to_string(), label: "Personal Item".to_string(), },
        OptionData { group: None, value: "business".to_string(), label: "A Business (your portion only)".to_string(), },
        OptionData { group: None, value: "money_owed_to_you".to_string(), label: "Money Owed To You".to_string(), },
        OptionData { group: None, value: "other_asset".to_string(), label: "Other Asset".to_string(), },
    ];

    let liability_options = vec![
        OptionData { group: None, value: "loan".to_string(), label: "Loan (mortgage, car, education, etc)".to_string(), },
        OptionData { group: None, value: "credit_card".to_string(), label: "Credit Card".to_string(), },
        OptionData { group: None, value: "other_liability".to_string(), label: "Other Liability".to_string(), },
    ];

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

        // <Select
        //     options=asset_options
        //     placeholder="Select an Asset"
        //     label="Assets"
        //     on_change=Callback::new(|val| log!("Selected: {}", val))
        // />
        // <Select />
        <Select>
            <SelectText>
                "Select An Option"
            </SelectText>

            <SelectOptions>
                <div></div>
            </SelectOptions>
        </Select>

        <br />

        // <AssetSelector />

        <p>"Add financial accounts, transfer money between accounts, and add bill pay alerts"</p>

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

// #[component]
// pub fn AssetSelector() -> impl IntoView {
//     // 1. Create a signal to hold the selected enum
//     // Default to the first item (BankAccount), or whatever you prefer
//     let (selected_asset_opt, set_selected_asset_opt) = signal(AssetCategory::BankAccount);

//     view! {
//         <div class="input-group">
//             <label for="category">"Asset Category"</label>
            
//             <select
//                 id="category"
//                 class="form-select" // Your styling class here
                
//                 // 2. Handle the change event
//                 on:change=move |ev| {
//                     // event_target_value helper gets the string value from the event
//                     let val = event_target_value(&ev);
//                     // Parse string back to Enum
//                     if let Ok(cat) = AssetCategory::from_str(&val) {
//                         set_selected_asset_opt.set(cat);
//                     }
//                 }
                
//                 // 3. Control the value so it syncs with signal (optional but good practice)
//                 prop:value=move || selected_asset_opt.get().option_value()
//             >
//                 // 4. Iterate over the Enum to generate options
//                 {AssetCategory::all().iter().map(|category| {
//                     view! {
//                         <option value={category.option_value()}>
//                             {category.option_text()}
//                         </option>
//                     }
//                 }).collect::<Vec<_>>()}
//             </select>

//             // Debugging: Show what is currently selected
//             <p style="margin-top: 10px; color: #666;">
//                 "Selected Enum: " 
//                 <strong>{move || format!("{:?}", selected_asset_opt.get())}</strong>
//             </p>
//         </div>
//     }
// }


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
                                                <p>"Type: " {asset.asset_type}</p>
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Asset {
  uuid: Uuid,
  asset_type: String,
//   asset_subtype: String,
  name: String,
  value: i64,
  login_url: String,
  sort_order: u64,
  created_at: String,
  calculated_bank_balance: bool,
}

#[server(name = CreateAsset, prefix = "/api/create-asset")]
pub async fn create_asset() -> Result<Asset, ServerFnError> {
    // Define the data as requested
    let asset = Asset { 
        // uuid: Some(Uuid::now_v7().to_string()),
        uuid: Uuid::now_v7(),
        asset_type: String::from("Bank Account"),
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
            asset_type: String::from("Bank Account"),
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
            asset_type: String::from("IRA"),
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
