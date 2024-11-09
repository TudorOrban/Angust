
use std::collections::HashMap;

use angust::rendering::router::router_proxy::{init_global_router, RouteConfiguration};


pub fn register_routes() {
    let mut routes = HashMap::new();
    
    let route_config = RouteConfiguration {
        routes,
        initial_route: None,
        cache_pages: true,
    };

    init_global_router(route_config);
}

    