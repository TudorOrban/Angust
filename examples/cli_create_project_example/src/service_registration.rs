
use angust::rendering::elements::service::service_registry::{initialize_service_registry, ServiceRegistry};


pub fn register_services() {
    let registry = ServiceRegistry::new();

    initialize_service_registry(registry);
}