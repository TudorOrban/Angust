use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;


static GLOBAL_ROUTER: Lazy<Mutex<Router>> = Lazy::new(|| {
    Mutex::new(Router::new(HashMap::new()))
});

pub fn init_global_router(routes: HashMap<String, String>) {
    let mut router = GLOBAL_ROUTER.lock().unwrap();
    *router = Router::new(routes);
}

pub fn navigate(route: &str) {
    let mut router = GLOBAL_ROUTER.lock().unwrap();
    router.navigate_to(route);
}

pub fn subscribe_to_current_route(callback: RouteChangeCallback) {
    let mut router = GLOBAL_ROUTER.lock().unwrap();
    router.subscribe(callback);
}

pub fn get_current_route() -> String {
    let router = GLOBAL_ROUTER.lock().unwrap();
    router.current_route.clone()
}

pub fn get_current_component_name() -> Option<String> {
    let router = GLOBAL_ROUTER.lock().unwrap();
    let comp_name_opt = router.routes.get(&router.current_route);
    if comp_name_opt.is_none() {
        return None;
    }

    Some(comp_name_opt.unwrap().clone())
}

type RouteChangeCallback = Arc<dyn Fn(&str, &str) + Send + Sync>;

pub struct Router {
    routes: HashMap<String, String>,
    current_route: String,
    #[allow(dead_code)]
    current_params: HashMap<String, String>,
    history: Vec<String>,
    subscribers: Vec<RouteChangeCallback>,
}

impl Router {
    fn new(routes: HashMap<String, String>) -> Self {
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
