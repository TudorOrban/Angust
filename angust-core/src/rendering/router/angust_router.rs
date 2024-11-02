use std::collections::HashMap;
use std::sync::Mutex;
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


pub struct Router {
    routes: HashMap<String, String>,
    current_route: String,
    current_params: HashMap<String, String>,
    history: Vec<String>,
}

impl Router {
    fn new(routes: HashMap<String, String>) -> Self {
        Router {
            routes,
            current_route: String::new(),
            current_params: HashMap::new(),
            history: Vec::new(),
        }
    }

    pub fn navigate_to(&mut self, route: &str) {
        if let Some(component_name) = self.routes.get(route) {
            self.history.push(route.to_string());
            self.current_route = route.to_string();
            println!("Navigating to route: {}, component: {}", route, component_name);
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
        }
    }
}

pub struct RouterComponent {

}