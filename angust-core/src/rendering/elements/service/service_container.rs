use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;
use once_cell::sync::OnceCell;

pub struct ServiceContainer {
    services: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        ServiceContainer {
            services: HashMap::new(),
        }
    }

    pub fn add_service<T: Any + Send + Sync + 'static>(&mut self, service_id: &str, service: T) {
        self.services.insert(service_id.to_string(), Arc::new(service));
    }

    pub fn get_service<T: Any + Send + Sync + 'static>(&self, service_id: &str) -> Option<&T> {
        self.services.get(service_id)?.as_ref().downcast_ref::<T>()
    }
}

static SERVICE_REGISTRY: OnceCell<Arc<ServiceContainer>> = OnceCell::new();

pub fn initialize_service_registry(registry: ServiceContainer) {
    let _ = SERVICE_REGISTRY.set(Arc::new(registry));
}

pub fn get_global_service<T: Any + Send + Sync + 'static>(service_id: &str) -> Option<&T> {
    SERVICE_REGISTRY.get().and_then(|registry| registry.get_service(service_id))
}
