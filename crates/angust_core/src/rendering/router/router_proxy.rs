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
    pub fn subscribe_to_current_route(&self, callback: RouteChangeCallback) {
        let mut router = GLOBAL_ROUTER.lock().unwrap();
        router.subscribe(callback);
    }

    pub fn navigate_to(&self, route: &str) {
        let mut router = GLOBAL_ROUTER.lock().unwrap();
        router.navigate_to(route);
    }

    pub fn get_current_params(&self) -> HashMap<String, String> {
        let router = GLOBAL_ROUTER.lock().unwrap();
        router.get_current_params().clone()
    }

    pub fn go_back(&self) {
        let mut router = GLOBAL_ROUTER.lock().unwrap();
        router.go_back();
    }

    pub fn go_forward(&self) {
        let mut router = GLOBAL_ROUTER.lock().unwrap();
        router.go_forward();
    }

    // Getters
    pub fn get_current_route(&self) -> String {
        let router = GLOBAL_ROUTER.lock().unwrap();
        router.get_current_route().clone()
    }

    pub fn get_current_component_name(&self) -> Option<String> {
        let router = GLOBAL_ROUTER.lock().unwrap();
        router.route_config.routes.get(&router.get_current_route()).cloned()
    }

    pub fn get_history(&self) -> Vec<(String, HashMap<String, String>)> {
        let router = GLOBAL_ROUTER.lock().unwrap();
        router.get_history().iter().cloned().collect()
    }
}


pub struct RouteConfiguration {
    pub routes: HashMap<String, String>,
    pub initial_route: Option<String>,
    pub cache_pages: bool,
}

impl Default for RouteConfiguration {
    fn default() -> Self {
        RouteConfiguration {
            routes: HashMap::new(),
            initial_route: None,
            cache_pages: false,
        }
    }
}

impl RouteConfiguration {
    pub fn match_route(&self, url_path: &str) -> Option<(&str, HashMap<String, String>)> {
        for (pattern, component_name) in &self.routes {
            let (is_match, params) = self.match_pattern(pattern, url_path);
            if is_match {
                return Some((component_name.as_str(), params));
            }
        }
        None
    }

    fn match_pattern(&self, pattern: &str, url_path: &str) -> (bool, HashMap<String, String>) {
        let mut params = HashMap::new();
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        let url_parts: Vec<&str> = url_path.split('/').collect();

        if pattern_parts.len() != url_parts.len() {
            return (false, params);
        }

        let mut is_match = true;
        for (pattern_part, url_part) in pattern_parts.iter().zip(url_parts.iter()) {
            if pattern_part.starts_with(':') {
                let param_name = &pattern_part[1..];
                params.insert(param_name.to_string(), url_part.to_string());
            } else if pattern_part != url_part {
                is_match = false;
                break;
            }
        }

        (is_match, params)
    }
}