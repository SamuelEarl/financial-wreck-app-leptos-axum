use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos::logging::{log, error};
use leptos::task::spawn_local;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use stylance::*;

use crate::components::{
    colors_and_sizes::{BtnVariant, Colors, Sizes},
    buttons::button::Button,
    dialogs::dialog::{ 
        Dialog, DialogTrigger, DialogContent, DialogBody, DialogHeader, 
        DialogTitle, DialogFooter, DialogClose
    },
    inputs::input::Input,
    selects::select::{Select},
};
use crate::utils::{
    format_currency::format_currency,
    dates::get_utc_iso_date,
};
use crate::pages::net_worth::{
    net_worth_article::NetWorthArticle,
    net_worth_models::{asset_options, liability_options},
};

import_style!(css, "net_worth.module.scss");

#[component]
pub fn NetWorth() -> impl IntoView {
    provide_meta_context();

    let (net_worth, set_net_worth) = signal(1_000_000);
    let (selected_asset, set_selected_asset) = signal("".to_string());

    view! {
        <Title text="Financial Wreck | Net Worth"/>

        <div class={css::h1_wrapper}>
            <h1 class={css::h1}>"Net Worth"</h1>
            <NetWorthArticle />
        </div>

        <h2>{move || format_currency(net_worth.get(), None)}</h2>

        <br />

        <Select
            options=asset_options()
            default_value="retirement_investment".to_string()
            placeholder="Select an Asset"
            btn_sizes=Some(Sizes {
                pv: Some(2),
                ph: Some(3),
                ..Default::default()
            })
            on_change=Callback::new(move |val: String| {
                log!("(net_worth) Selected: {}", val);
                set_selected_asset.set(val);
            })
        />

        <div>"Selected Asset: " { move || selected_asset.get() }</div>

        <br />

        <p>"Add financial accounts, transfer money between accounts, and add bill pay alerts"</p>

        <NWItemsList nw_item_type="asset".to_string() />
        
        <br />

        <NWItemsList nw_item_type="liability".to_string() />
    }
}


// The NWItemsList component displays the Net Worth items (e.g. Assets, Liabilities) that the user has added to their account.
#[component]
pub fn NWItemsList(nw_item_type: String) -> impl IntoView {
    let nw_items_type = {
        if nw_item_type == "asset" {
            "Assets".to_string()
        } else {
            "Liabilities".to_string()
        }
    };
    // 1. Create the Resource
    // The first argument `|| ()` is the "dependency." 
    // Since it's empty, this resource runs exactly once (on load).
    // Clone the string once to get it into the closure environment.
    let type_for_get_resource = nw_items_type.clone();
    // The fetcher (second argument) is an Fn, so you need to clone inside it.
    let items_resource = Resource::new(
        // Leptos passes the result of the first closure (the cloned String) into the second closure (where the call to get_items() is located).
        move || type_for_get_resource.clone(),
        |t| get_items(t)
    );

    provide_context(items_resource);

    let type_for_create_resource = nw_items_type.clone();

    view! {
        <div class={css::al_wrapper}>
            <h2 class={css::h2}>{nw_items_type}</h2>
            <div class={css::btns_container}>
                <AddNWItemDialog nw_item_type=nw_item_type />
            </div>
        </div>

        // 2. Wrap in Suspense to handle the "Loading..." state
        <Suspense fallback=move || view! { <p>"Loading {nw_items_type}..."</p> }>
            
            // 3. Read the resource
            // We use ErrorBoundary to handle if the server function fails (Result::Err)
            <ErrorBoundary fallback=|_| view! { <p>"Something went wrong."</p> }>
                {move || {
                    // .get() returns Option<Result<Vec<NWItems>, Error>>
                    // .map handles the "Loaded" state
                    items_resource.get().map(|result| {
                        match result {
                            Ok(items) => view! {
                                <div class="asset-grid">
                                    // 4. Iterate over the Vec<NWItem>
                                    // <For> is efficient for lists that might change
                                    // Note: We use items.clone() here because the resource
                                    // owns the data. This is standard for small lists.
                                    <For
                                        each=move || items.clone()
                                        key=|item| item.uuid // Use UUID as the unique key
                                        children=|item| view! {
                                            <div class="item-card">
                                                <h4>{item.name}</h4>
                                                <p>"Value: $" {item.value}</p>
                                                <p>"Type: " {item.item_type}</p>
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


#[component]
pub fn AddNWItemDialog(nw_item_type: String) -> impl IntoView {
    let items_resource = use_context::<Resource<Result<Vec<NWItem>, ServerFnError>>>()
    .expect("resource not found");

    // The closure needs to clone the data it returns.
    let item_options = {
        if nw_item_type == "asset" {
            asset_options.clone()
        } else {
            liability_options.clone()
        }
    };

    let select_label = {
        if nw_item_type == "asset" {
            "Select an asset type"
        } else {
            "Select a liability type"
        }
    };

    let name_label = {
        if nw_item_type == "asset" {
            "Name of asset"
        } else {
            "Name of liability"
        }
    };

    let default_val = StoredValue::new(if nw_item_type == "asset" {
        "bank_account".to_string()
    } else {
        "loan".to_string()
    });

    let (selected_item, set_selected_item) = signal("".to_string());

    // Store the string in the Leptos runtime.
    // This returns a 'StoredValue<String>' which is Copy.
    let title_type = StoredValue::new(nw_item_type);

    view! {
        <Dialog>
            <DialogTrigger
                variant={BtnVariant::Primary}
                sizes=Some(Sizes {
                    pv: Some(0),
                    ph: Some(2),
                    ..Default::default()
                })
            >
                "Add"
            </DialogTrigger>

            <DialogContent>
                <DialogHeader>
                    <DialogTitle>
                        // .with_value() lets you access the string without moving it.
                        "Add " { move || title_type.with_value(|t| t.clone()) }
                    </DialogTitle>
                </DialogHeader>

                <DialogBody>
                    <label>
                        {select_label}
                        <Select
                            options=item_options()
                            default_value=default_val.get_value()
                            btn_sizes=Some(Sizes {
                                pv: Some(2),
                                ph: Some(3),
                                ..Default::default()
                            })
                            on_change=Callback::new(move |val: String| {
                                log!("(net_worth) Selected: {}", val);
                                set_selected_item.set(val);
                            })
                        />
                    </label>

                    <br/>

                    <label>
                        {name_label}
                        <Input
                            attr:placeholder="Default input"
                        />
                    </label>
                </DialogBody>

                <DialogFooter>
                    <DialogClose 
                        colors=Some(Colors {
                            bg: "transparent".to_string(),
                            fg: "var(--secondary-bg)".to_string(),
                            br: "var(--secondary-bg)".to_string(),
                            ol: "var(--secondary-bg)".to_string(),
                        })
                    >
                        "Cancel"
                    </DialogClose>
                    <DialogClose
                        variant={BtnVariant::Secondary}
                        // TODO: Add a disabled state to the <DialogClose> component so this disabled attribute actually works.
                        // Disable if nothing is selected
                        attr:disabled=move || selected_item.get().is_empty()
                        on:click=move |_| {
                            // 1. Get the current values from our signals/storage
                            let item_type = title_type.get_value(); // "asset" or "liability"
                            let item_val = selected_item.get();    // The value from the Select
                            
                            // 2. Fire and forget the server call
                            spawn_local(async move {
                                log!("Creating {}: {}", item_type, item_val);
                                
                                match create_item(item_type).await {
                                    Ok(new_item) => {
                                        // 3. Update the context resource we grabbed at the top of the component
                                        items_resource.update(|current_state| {
                                            // current_state is &mut Option<Result<Vec<NWItem>, Error>>
                                            // We only want to push if we currently have a valid list.
                                            if let Some(Ok(list)) = current_state {
                                                list.push(new_item);
                                            }
                                        });
                                    },
                                    Err(e) => error!("Failed to create item: {}", e),
                                }
                            });
                        }
                    >
                        "Add " { move || title_type.with_value(|t| t.clone()) }
                    </DialogClose>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NWItem {
  uuid: Uuid,
  item_type: String,
//   asset_subtype: String,
  name: String,
  value: i64,
  login_url: String,
  sort_order: u64,
  created_at: String,
  calculated_bank_balance: bool,
}

#[server(name = CreateItem, prefix = "/api/create-item")]
pub async fn create_item(nw_items_type: String) -> Result<NWItem, ServerFnError> {
    // Define the data as requested
    let asset = NWItem { 
        // uuid: Some(Uuid::now_v7().to_string()),
        uuid: Uuid::now_v7(),
        item_type: String::from("Bank Account"),
        name: String::from("Capital One Savings"),
        value: 1_000_000, // $10,000.00
        login_url: String::from("https://capitalone.com"),
        sort_order: 0,
        created_at: get_utc_iso_date(),
        calculated_bank_balance: true,
    };

    // TODO: Save the new asset to the database here...

    Ok(asset)
}

#[server(name = GetItems, prefix = "/api/get-items")]
pub async fn get_items(nw_items_type: String) -> Result<Vec<NWItem>, ServerFnError> {
    // Define the data as requested
    let assets = vec![
        NWItem { 
            // uuid: Some(Uuid::now_v7().to_string()),
            uuid: Uuid::now_v7(),
            item_type: String::from("Bank Account"),
            name: String::from("Capital One Savings"),
            value: 1_000_000, // $10,000.00
            login_url: String::from("https://capitalone.com"),
            sort_order: 0,
            created_at: get_utc_iso_date(),
            calculated_bank_balance: true,
        },
        NWItem { 
            // uuid: Some(Uuid::now_v7().to_string()),
            uuid: Uuid::now_v7(),
            item_type: String::from("IRA"),
            name: String::from("Vangard"),
            value: 10_000_000, // $10,000.00
            login_url: String::from("https://vangard.com"),
            sort_order: 1,
            created_at: get_utc_iso_date(),
            calculated_bank_balance: false,
        },
    ];

    let liabilities = vec![
        NWItem { 
            // uuid: Some(Uuid::now_v7().to_string()),
            uuid: Uuid::now_v7(),
            item_type: "Loan".to_string(),
            name: "Student Loans".to_string(),
            value: 1_000_000, // $10,000.00
            login_url: "https://finaid.edu".to_string(),
            sort_order: 0,
            created_at: get_utc_iso_date(),
            calculated_bank_balance: false,
        },
        NWItem { 
            // uuid: Some(Uuid::now_v7().to_string()),
            uuid: Uuid::now_v7(),
            item_type: "Credit Card".to_string(),
            name: "AMEX".to_string(),
            value: 10_000_000, // $10,000.00
            login_url: "https://amex.com".to_string(),
            sort_order: 1,
            created_at: get_utc_iso_date(),
            calculated_bank_balance: false,
        },
    ];

    if nw_items_type == "Assets" {
        return Ok(assets);
    } else {
        return Ok(liabilities);
    }
}
