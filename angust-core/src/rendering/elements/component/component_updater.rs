// use std::sync::{Arc, Mutex};
// use crate::rendering::elements::component::component::Component;
// use std::collections::HashMap;

//     static mut COMPONENT_MAP: HashMap<String, *mut dyn Component> = HashMap::new();

//     pub fn register_component(id: String, component: &mut dyn Component) {
//         unsafe {
//             COMPONENT_MAP.insert(id, component as *mut dyn Component);
//         }
//     }

//     pub fn update_component_by_id(id: &str) {
//         unsafe {
//             if let Some(component) = COMPONENT_MAP.get(id) {
//                 let component = &mut **component;
//                 component.recompute_layout();
//                 component.render();
//             }
//         }
//     }