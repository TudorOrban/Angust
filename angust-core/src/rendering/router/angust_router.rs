use std::{collections::HashMap, sync::Arc};


pub struct Router {
    pub routes: HashMap<String, String>,
    pub current_route: String,
    #[allow(dead_code)]
    current_params: HashMap<String, String>,
    history: Vec<String>,
    subscribers: Vec<RouteChangeCallback>,
}

impl Router {
    pub fn new(routes: HashMap<String, String>) -> Self {
        Router {
            routes,
            ..Default::default()
        }
    }

    pub fn navigate_to(&mut self, route: &str) {
        let component_name_opt = self.routes.get(route);
        if component_name_opt.is_none() {
            return;
        }
        let component_name = component_name_opt.unwrap();

        self.history.push(route.to_string());
        self.current_route = route.to_string();

        self.notify_subscribers(route, component_name);
    }

    pub fn subscribe(&mut self, callback: RouteChangeCallback) {
        self.subscribers.push(callback);
    }

    fn notify_subscribers(&self, route: &str, component_name: &str) {
        for subscriber in &self.subscribers {
            subscriber(route, component_name);
        }
    }
}

pub type RouteChangeCallback = Arc<dyn Fn(&str, &str) + Send + Sync>;

impl Default for Router {
    fn default() -> Self {
        Router {
            routes: HashMap::new(),
            current_route: String::from(""),
            current_params: HashMap::new(),
            history: Vec::new(),
            subscribers: Vec::new(),
        }
    }
}
