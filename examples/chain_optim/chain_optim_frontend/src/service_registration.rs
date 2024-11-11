use crate::app::features::products::services::product_service::product_service::ProductService;

use angust::rendering::elements::service::service_registry::{initialize_service_registry, ServiceRegistry};


pub fn register_services() {
    let mut registry = ServiceRegistry::new();
    registry.add_service("product-service", ProductService::new());

    initialize_service_registry(registry);
}   
    
    