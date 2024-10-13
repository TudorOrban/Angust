use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

use crate::rendering::elements::element::Element;

use super::component::Component;


lazy_static! {
    static ref COMPONENT_REGISTRY: Mutex<HashMap<String, Box<dyn Fn() -> Box<dyn Element> + Send>>> = Mutex::new(HashMap::new());
}

pub fn register_component<State>(name: String, factory: Box<dyn Fn() -> Component<State> + Send>)
where 
    State: 'static {
        let wrapped_factory = Box::new(move || {
            let component = factory();
            Box::new(component) as Box<dyn Element>
        });

        COMPONENT_REGISTRY.lock().unwrap().insert(name, wrapped_factory);
    }

pub fn create_component(name: &str) -> Option<Box<dyn Element>> {
    let registry = COMPONENT_REGISTRY.lock().unwrap();
    registry.get(name).and_then(|factory| Some(factory()))
}