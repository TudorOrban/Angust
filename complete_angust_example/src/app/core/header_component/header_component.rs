
use std::{collections::HashMap, any::Any};

use angust::{rendering::elements::component::{
    component::Component, component_factory_registry::ComponentFactory, functions::component_functions::ComponentFunctions, state::{
        reactivity::{ComponentEvent, ReactiveField, ReactiveState}, 
        reflectivity::ReflectiveState
    }
}, wrap_fn_mut};
use angust_macros::component_state;

use crate::app::app_component::UIItem;



#[component_state]
pub struct HeaderComponentState {
    content: String,
    count: f64,
    item: UIItem,
}

pub struct HeaderComponent {
    
}

impl HeaderComponentState {

    fn set_content(&mut self, content: String) {
        self.content = content;
    }

    fn set_count(&mut self, count: f64) {
        self.count = count;
    }

    fn set_item(&mut self, item: UIItem) {
        self.item = item;
    }
}

impl HeaderComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let state_factory = || HeaderComponentState::new(
            String::from("Hello from header-component"),
            20.0,
            UIItem::new(
                String::from("Label"),
                String::from("Value"),
                vec![],
            ),
        );

        registry.insert("header-component".to_string(), Box::new(move || {
            let mut component = Component::new(
                "header-component".to_string(),
                "src/app/core/header_component/header_component.html".to_string(),
                state_factory() 
            );

            let component_functions: ComponentFunctions<HeaderComponentState> = ComponentFunctions::new(
                vec![], vec![], vec![], vec![], vec![],
                vec![
                    ("set_content", wrap_fn_mut!(HeaderComponentState, HeaderComponentState::set_content, String)),
                    ("set_count", wrap_fn_mut!(HeaderComponentState, HeaderComponentState::set_count, f64)),
                    // ("set_item", wrap_fn_mut!(HeaderComponentState, HeaderComponentState::set_item, UIItem)),
                ],
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }
}
    