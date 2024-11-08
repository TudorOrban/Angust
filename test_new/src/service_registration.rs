
use angust::rendering::elements::service::service_container::{initialize_service_registry, ServiceContainer};


pub fn register_services() {
    let mut registry = ServiceContainer::new();

    initialize_service_registry(registry);
}   
    
    