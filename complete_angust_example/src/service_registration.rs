use angust::rendering::elements::component::service_container::{initialize_service_registry, ServiceContainer};

use crate::ProductService;


pub fn register_services() {
    let mut registry = ServiceContainer::new();
    
    registry.add_service("ProductService", ProductService::new());

    initialize_service_registry(registry);
}