use std::{collections::{HashMap, VecDeque}, sync::Arc};

use super::router_proxy::RouteConfiguration;


pub struct Router {
    pub route_config: RouteConfiguration,
    current_route: String,
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

        if !self.current_route.is_empty() {
            self.history.push_back((self.current_route.clone(), self.current_params.clone()));
        }
        self.forward_stack.clear();

        self.current_route = route.to_string();
        self.current_params = params;

        self.notify_subscribers(route, component_name);
    }

    pub fn go_back(&mut self) {
        let (previous_route, previous_params) = match self.history.pop_back() {
            Some(route) => route,
            None => return,
        };

        self.forward_stack.push_front((self.current_route.clone(), self.current_params.clone()));
        self.current_route = previous_route;
        self.current_params = previous_params;

        if let Some((component_name, _)) = self.route_config.match_route(&self.current_route) {
            self.notify_subscribers(&self.current_route, component_name);
        }
    }
    
    pub fn go_forward(&mut self) {
        let (next_route, next_params) = match self.forward_stack.pop_front() {
            Some(route) => route,
            None => return,
        };

        self.history.push_back((self.current_route.clone(), self.current_params.clone()));
        self.current_route = next_route;
        self.current_params = next_params;

        if let Some((component_name, _)) = self.route_config.match_route(&self.current_route) {
            self.notify_subscribers(&self.current_route, component_name);
        }
    }

    pub fn subscribe(&mut self, callback: RouteChangeCallback) {
        self.subscribers.push(callback);
    }

    fn notify_subscribers(&self, route: &str, component_name: &str) {
        for subscriber in &self.subscribers {
            subscriber(route, component_name);
        }
    }

    // Getters
    pub fn get_current_route(&self) -> String {
        self.current_route.clone()
    }
    
    pub fn get_current_params(&self) -> HashMap<String, String> {
        self.current_params.clone()
    }

    pub fn get_history(&self) -> VecDeque<(String, HashMap<String, String>)> {
        self.history.clone()
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
