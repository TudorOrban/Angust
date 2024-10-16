
// use std::collections::HashMap;

// use angust::{component_state, rendering::elements::component::{component::Component, component_factory_registry::ComponentFactory}};


// pub struct HeaderComponent {
//     component: Component<HeaderComponentState>,    
// }


// component_state! {
//     HeaderComponentState {
//         content: String,
//     }
// }

// impl HeaderComponentState {
//     // pub fn new() -> HeaderComponentState {
//     //     HeaderComponentState {
//     //         content: "Hello from header-component".to_string(),
//     //     }
//     // }
// }

// impl HeaderComponent {
//     pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
//         let state_factory = || HeaderComponentState::new(
//             String::from("Hello from header-component")
//         );

//         registry.insert("header-component".to_string(), Box::new(move || {
//             Box::new(
//                 Component::new(
//                     "header-component".to_string(),
//                     "src/app/core/header_component/header_component.html".to_string(),
//                     state_factory() 
//                 )
//             )
//         }));
//     }
// }
    