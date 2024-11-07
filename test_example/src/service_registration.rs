
use angust::rendering::elements::service::service_container::{initialize_service_registry, ServiceContainer};

use crate::app::core::services::product_service::ProductService;


pub fn register_services() {
    let mut registry = ServiceContainer::new();
    
    registry.add_service("ProductService", ProductService::new());

    initialize_service_registry(registry);
}