use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes, ParentRoute},
    // StaticSegment,
    path,
};

use crate::pages::auth::sign_in::SignIn;
use crate::pages::layout::Layout;
use crate::pages::dashboard::dashboard::Dashboard;
use crate::pages::goals::goals::Goals;
use crate::pages::net_worth::net_worth::NetWorth;
use crate::pages::bank_accounts::{
    bank_transactions_and_budgets_list::bankTransactionsAndBudgetsList,
    transactions_budget_wrapper::TransactionsBudgetWrapper,
    transactions::Transactions, 
    budget::Budget, 
};
// use crate::pages::errors::not_found::NotFound;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                // <Route path=StaticSegment("") view=HomePage />
                <Route path=path!("/") view=SignIn />
                <ParentRoute path=path!("/") view=Layout>
                    <Route path=path!("/dashboard") view=Dashboard />
                    <Route path=path!("/goals") view=Goals />
                    <Route path=path!("/net-worth") view=NetWorth />
                    <Route path=path!("/bank-transactions-and-budgets-list") view=bankTransactionsAndBudgetsList />
                    <ParentRoute
                        path=path!("/bank-accounts/:uuid/:account_name/:year/:month_name")
                        view=TransactionsBudgetWrapper
                    >
                        <Route path=path!("/transactions") view=Transactions />
                        <Route path=path!("/budget") view=Budget />
                    </ParentRoute>
                    // I have recorded the fix for the catch-all route in the src/pages/errors/404_ROUTE_FIX.md file. I can implement it after I have implemented the server code.
                    // Key Details
                    // path="*any": The * tells the router this is a wildcard. The text after the asterisk (e.g., any) is the name of the parameter if you wanted to access the bad URL string (e.g., to print "The page /foo/bar does not exist").
                    // ResponseOptions: This is essential for SEO. If a bot crawls a bad link and gets a 200 OK, it may index your error page as valid content. Setting the status to NOT_FOUND prevents this.
                    // <Route path=path!("*any") view=NotFound />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
