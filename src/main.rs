// use doteny::dotenv;
// use std::env;
// use std::sync::Arc;
// use falkordb::{
//     // FalkorClient,
//     FalkorClientBuilder,
//     FalkorConnectionInfo
// };

// #[derive(Clone, Debug)]
// pub struct DbClient(pub Arc<FalkorClient>);


#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use financial_wreck_app_leptos_axum::app::*;

    // // Load the .env file
    // // We use .ok() to ignore errors (e.g., if the file is missing in production 
    // // because vars are passed via Docker/OS).
    // dotenv().ok();

    // // Ensure you have a FalkorDB instance running.
    // let connection_info: FalkorConnectionInfo = env::var("FALKOR_ENDPOINT")
    //     .try_into()
    //     .expect("Invalid FalkorDB URL");

    // let client = FalkorClientBuilder::new_async()
    //     .with_connection_info(connection_info)
    //     .build()
    //     .await
    //     .expect("Failed to build FalkorDB client");

    // // Select the graph.
    // let mut graph_instance = client.select_graph(env::var("FALKOR_GRAPH"));

    // // Wrap the database instance in the DbClient struct and Arc.
    // let db_instance = DbClient(Arc::new(client));

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(
            &leptos_options, 
            routes, 
            {
                // CRITICAL: Clone the DB instance *outside* the closure 
                // so it moves into the closure's scope.
                // let db = db_instance.clone();

                // let mut graph = graph_instance.clone();

                let leptos_options = leptos_options.clone();

                // This closure runs for EVERY request.
                move || shell(leptos_options.clone())
                // move || {
                //     // provide_context(db.clone());
                //     // provide_context(graph.clone());
                //     shell(leptos_options.clone())
                // }
            }
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
