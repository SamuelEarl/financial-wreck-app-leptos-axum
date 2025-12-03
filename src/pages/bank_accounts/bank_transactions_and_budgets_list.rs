use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::A;
use stylance::*;
// use chrono::{Datelike, Local}; // Import Datelike to use .year()

// use crate::pages::sidebar_nav::NavLink;

import_style!(css, "bank_transactions_and_budgets_list.module.scss");

#[derive(Debug)]
pub struct BankAccount {
    pub name: String,
    pub transactions_url: String,
    pub budget_url: String,
}

#[component]
pub fn bankTransactionsAndBudgetsList() -> impl IntoView {
    provide_meta_context();

    let bank_accounts: [BankAccount; 2] = [
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

    view! {
        <Title text="Financial Wreck | Bank Accounts"/>

        <h1>"Bank Transactions & Budgets"</h1>

        <p>"Select the transactions or the budget for the bank account you want to view:"</p>

        // /bank-accounts/:uuid/:account_name/:year/:month_name/transactions
        // /bank-accounts/:uuid/:account_name/:year/:month_name/budgets
        <ul class={css::bank_accounts_list}>
            {
                bank_accounts.into_iter()
                    .map(|account| view! {
                        <li>
                            <div class={css::bank_account_name}>{account.name}</div> 
                            <div class={css::transactions_budget_container}>
                                <A href={account.transactions_url}>Transactions</A>
                                <A href={account.budget_url}>Budget</A>
                            </div>
                        </li>
                    })
                    .collect_view()
            }
        </ul>
    }
}
