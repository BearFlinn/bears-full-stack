
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::{get, post};
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use bears_full_stack::view::App;
    use bears_full_stack::controller::{file_and_error_handler, apis::*};

    
    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let apis = Router::new()
        .route("/ur-mom", get(ur_mom))
        .route("/writing-sample", post(submit_form))
        .route("/sample-titles", get(get_titles));

    let view_router = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let app = Router::new()
        .nest("/api", apis)
        .merge(view_router);
       

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("Listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}


#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
