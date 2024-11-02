use std::{any::Any, collections::HashMap};


pub struct ServiceContainer {
    services: HashMap<String, Box<dyn Any>>,
}


impl ServiceContainer {
    pub fn new() -> Self {
        ServiceContainer {
            services: HashMap::new(),
        }
    }

    pub fn add_service<T: Any + 'static>(&mut self, service_id: &str, service: T) {
        self.services.insert(service_id.to_string(), Box::new(service));
    }

    pub fn get_service<T: Any + 'static>(&self, service_id: &str) -> Option<&T> {
        self.services.get(service_id)?.as_ref().downcast_ref::<T>()
    }
}