
use std::collections::HashMap;

use angust::rendering::router::router_proxy::{init_global_router, RouteConfiguration};


pub fn register_routes() {
    let mut routes = HashMap::new();

    routes.insert("Home".to_string(), "home-component".to_string());
    routes.insert("Dashboard".to_string(), "dashboard-component".to_string());
    routes.insert("Products".to_string(), "products-component".to_string());
    routes.insert("Settings".to_string(), "settings-component".to_string());
    
    let route_config = RouteConfiguration {
        routes,
        initial_route: Some("Home".to_string()),
        cache_pages: true,
    };

    init_global_router(route_config);
}

    