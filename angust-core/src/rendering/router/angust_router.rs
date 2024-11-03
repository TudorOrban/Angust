use std::{collections::{HashMap, VecDeque}, sync::Arc};

use super::router_proxy::RouteConfiguration;


pub struct Router {
    pub route_config: RouteConfiguration,
    pub current_route: String,
    current_params: HashMap<String, String>,
    history: VecDeque<(String, HashMap<String, String>)>,
    forward_stack: VecDeque<(String, HashMap<String, String>)>,  
    subscribers: Vec<RouteChangeCallback>,
}

impl Router {
    pub fn new(route_config: RouteConfiguration) -> Self {
        let current_route = route_config.initial_route.clone().unwrap_or_else(|| String::from(""));
        Router {
            route_config,
            current_route,
            ..Default::default()
        }
    }

    pub fn navigate_to(&mut self, route: &str) {
        let component_name_opt = self.route_config.match_route(route);
        if component_name_opt.is_none() {
            return;
        }
        let (component_name, params) = component_name_opt.unwrap();

        self.forward_stack.clear();
        if !self.current_route.is_empty() {
            self.history.push_back((self.current_route.clone(), params.clone()));
        }

        self.current_route = route.to_string();
        self.current_params = params;

        self.notify_subscribers(route, component_name);
    }

    pub fn get_current_params(&self) -> HashMap<String, String> {
        self.current_params.clone()
    }

    pub fn go_back(&mut self) {
        let previous_route = match self.history.pop_back() {
            Some(route) => route,
            None => return,
        };
        self.forward_stack.push_front((self.current_route.clone(), self.current_params.clone()));
        self.current_route = previous_route.0;
        self.current_params = previous_route.1;
        if let Some(component_name) = self.route_config.routes.get(&self.current_route) {
            self.notify_subscribers(&self.current_route, component_name);
        }
    }
    
    pub fn go_forward(&mut self) {
        let next_route = match self.forward_stack.pop_front() {
            Some(route) => route,
            None => return,
        };
        self.history.push_back((self.current_route.clone(), self.current_params.clone()));
        self.current_route = next_route.0;
        self.current_params = next_route.1;
        if let Some(component_name) = self.route_config.routes.get(&self.current_route) {
            self.notify_subscribers(&self.current_route, component_name);
        }
    }

    pub fn get_history(&self) -> VecDeque<(String, HashMap<String, String>)> {
        self.history.clone()
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
            route_config: RouteConfiguration::default(),
            current_route: String::from(""),
            current_params: HashMap::new(),
            history: VecDeque::new(),
            forward_stack: VecDeque::new(),
            subscribers: Vec::new(),
        }
    }
}
