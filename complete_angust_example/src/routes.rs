use std::collections::HashMap;

use angust::rendering::router::router_proxy::{init_global_router, RouteConfiguration};


pub fn register_routes() {
    let mut routes = HashMap::new();
    routes.insert(String::from("/header"), String::from("header-component"));
    routes.insert(String::from("/main-menu/:product_id"), String::from("main-menu-component"));

    let route_config = RouteConfiguration {
        routes,
        initial_route: Some(String::from("/header")),
        cache_pages: true,
    };

    init_global_router(route_config);
}