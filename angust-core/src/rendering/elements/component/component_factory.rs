use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::OnceCell;

use crate::rendering::elements::element::Element;

/*
 * Registry for component factories, populated at runtime by the client application 
 * and used by Angust to create custom components.
 */
pub type ComponentFactory = Box<dyn Fn() -> Box<dyn Element> + Send + Sync>;
static COMPONENT_REGISTRY: OnceCell<Arc<HashMap<String, ComponentFactory>>> = OnceCell::new();

pub fn initialize_registry(registry: HashMap<String, ComponentFactory>) {
    let immutable_registry = Arc::new(registry);
    let _ = COMPONENT_REGISTRY.set(immutable_registry);
}

pub fn create_component(name: &str) -> Option<Box<dyn Element>> {
    COMPONENT_REGISTRY.get().and_then(|registry| {
        registry.get(name).and_then(|factory| Some(factory()))
    })
}
