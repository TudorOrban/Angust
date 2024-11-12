use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;
use once_cell::sync::OnceCell;

pub struct ServiceRegistry {
    services: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        ServiceRegistry {
            services: HashMap::new(),
        }
    }

    pub fn add_service<T: Any + Send + Sync + 'static>(&mut self, service_name: &str, service: T) {
        self.services.insert(service_name.to_string(), Arc::new(service));
    }

    pub fn get_service<T: Any + Send + Sync + 'static>(&self, service_name: &str) -> Option<&T> {
        self.services.get(service_name)?.as_ref().downcast_ref::<T>()
    }
}

static SERVICE_REGISTRY: OnceCell<Arc<ServiceRegistry>> = OnceCell::new();

pub fn initialize_service_registry(registry: ServiceRegistry) {
    let _ = SERVICE_REGISTRY.set(Arc::new(registry));
}

pub fn get_global_service<T: Any + Send + Sync + 'static>(service_name: &str) -> Option<&T> {
    SERVICE_REGISTRY.get().and_then(|registry| registry.get_service(service_name))
}
