use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ParentRoute},
    // StaticSegment,
    path,
};
// use stylance::*;

use crate::pages::auth::sign_in::SignIn;
use crate::pages::layout::Layout;
use crate::pages::dashboard::dashboard::Dashboard;
use crate::pages::goals::goals::Goals;
use crate::pages::bank_accounts::{
    transactions::Transactions, 
    budget::Budget, 
    bank_account_layout::BankAccountLayout
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/financial-wreck-app.css"/>

        // sets the document title
        <Title text="Financial Wreck"/>

        // content for this welcome page
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                // <Route path=StaticSegment("") view=HomePage />
                <Route path=path!("/") view=SignIn />
                <ParentRoute path=path!("/") view=Layout>
                    <Route path=path!("/dashboard") view=Dashboard />
                    <Route path=path!("/goals") view=Goals />
                    <ParentRoute path=path!("/bank-accounts/:uuid/:account_name/:year/:month_name") view=BankAccountLayout>
                        <Route path=path!("/transactions") view=Transactions />
                        <Route path=path!("/budget") view=Budget />
                    </ParentRoute>
                </ParentRoute>
                // This catch-all route for 404 - Not Found pages is causing errors. So I need to figure out how to implement this correctly.
                // <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </Router>
    }
}

// /// Renders the home page of your application.
// #[component]
// fn HomePage() -> impl IntoView {
//     // Creates a reactive value to update the button
//     let count = RwSignal::new(0);
//     let on_click = move |_| *count.write() += 1;

//     view! {
//         <h1>"Welcome to Leptos!"</h1>
//         <button on:click=on_click>"Click Me: " {count}</button>
//     }
// }
