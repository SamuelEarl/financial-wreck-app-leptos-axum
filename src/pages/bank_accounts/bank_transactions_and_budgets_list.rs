use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::A;
use serde::{Deserialize, Serialize};
use stylance::*;
// use chrono::{Datelike, Local}; // Import Datelike to use .year()

import_style!(css, "bank_transactions_and_budgets_list.module.scss");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BankAccount {
    pub name: String,
    pub transactions_url: String,
    pub budget_url: String,
}

#[component]
pub fn bankTransactionsAndBudgetsList() -> impl IntoView {
    provide_meta_context();

    // Fetch data on page load.
    // Create the Resource
    // The first argument `|| ()` is the "source" signal. Since it's unit `()`,
    // it runs exactly once when the component loads.
    let accounts_resource = Resource::new(
        || (), 
        |_| get_bank_accounts()
    );

    view! {
        <Title text="Financial Wreck | Bank Accounts"/>

        <h1>"Bank Transactions & Budgets"</h1>

        <p>"Select the transactions or the budget for the bank account you want to view:"</p>

        // Wrap in Suspense to handle the "loading" state.
        <Suspense fallback=move || view! { <p>"Loading bank accounts..."</p> }>
            {move || {
                // Read the resource
                // .get() returns Option<Result<Vec<...>>>
                accounts_resource.get().map(|result| match result {
                    Ok(bank_accounts) => view! {
                        <ul class={css::bank_accounts_list}>
                            {
                                bank_accounts.into_iter().map(|account| view! {
                                    <li class={css::bank_accounts_list_item}>
                                        <div class={css::bank_account_name}>{account.name}</div> 
                                        <div class={css::transactions_budget_container}>
                                            // URLs to the Transactions and Budgets tabs/pages.
                                            // /bank-accounts/:uuid/:account_name/:year/:month_name/transactions
                                            // /bank-accounts/:uuid/:account_name/:year/:month_name/budgets
                                            <A href={account.transactions_url}>Transactions</A>
                                            <A href={account.budget_url}>Budget</A>
                                        </div>
                                    </li>
                                })
                                .collect_view()
                            }
                        </ul>
                    }.into_any(),
                    Err(e) => view! { <p class="error">"Error: " {e.to_string()}</p> }.into_any(),
                })
            }}
        </Suspense>
    }
}

#[server(name = GetBankAccounts, prefix = "/api/bank-accounts")]
pub async fn get_bank_accounts() -> Result<Vec<BankAccount>, ServerFnError> {
    // Define the data as requested
    let bank_accounts = vec![
        BankAccount { 
            name: String::from("USAA Checking"), 
            transactions_url: String::from("1234/USAA Checking/2025/december/transactions"),
            budget_url: String::from("1234/USAA Checking/2025/december/budget"),
        },
        BankAccount { 
            name: String::from("USAA Savings"), 
            transactions_url: String::from("5678/USAA Savings/2025/december/transactions"),
            budget_url: String::from("5678/USAA Savings/2025/december/budget"),
        },
    ];

    Ok(bank_accounts)
}
