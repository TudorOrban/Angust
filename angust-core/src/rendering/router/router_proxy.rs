use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

use super::angust_router::{RouteChangeCallback, Router};

/*
 * Proxy allowing to interact with the global router
 */
static GLOBAL_ROUTER: Lazy<Mutex<Router>> = Lazy::new(|| {
    Mutex::new(Router::new(RouteConfiguration::default()))
});

pub fn init_global_router(route_config: RouteConfiguration) {
    let mut router = GLOBAL_ROUTER.lock().unwrap();
    *router = Router::new(route_config);
}

pub fn get_router() -> RouterProxy {
    RouterProxy
}

pub struct RouterProxy;

impl RouterProxy {
    pub fn get_current_route(&self) -> String {
        let router = GLOBAL_ROUTER.lock().unwrap();
        router.current_route.clone()
    }

    pub fn get_current_component_name(&self) -> Option<String> {
        let router = GLOBAL_ROUTER.lock().unwrap();
        router.route_config.routes.get(&router.current_route).cloned()
    }

    pub fn subscribe_to_current_route(&self, callback: RouteChangeCallback) {
        let mut router = GLOBAL_ROUTER.lock().unwrap();
        router.subscribe(callback);
    }

    pub fn navigate(&self, route: &str) {
        let mut router = GLOBAL_ROUTER.lock().unwrap();
        router.navigate_to(route);
    }
}


pub struct RouteConfiguration {
    pub routes: HashMap<String, String>,
    pub initial_route: Option<String>,
}

impl Default for RouteConfiguration {
    fn default() -> Self {
        RouteConfiguration {
            routes: HashMap::new(),
            initial_route: None,
        }
    }
}