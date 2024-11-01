
use std::{collections::HashMap, any::Any};

use angust::{rendering::elements::component::{
    component::Component, component_factory_registry::ComponentFactory, functions::component_functions::ComponentFunctions, state::{
        reactivity::{ComponentEvent, ReactiveField, ReactiveState}, 
        reflectivity::ReflectiveState
    }
}, wrap_fn_mut};
use angust_macros::component_state;



#[component_state]
pub struct HeaderComponentState {
    content: String,
}

pub struct HeaderComponent {
    
}

impl HeaderComponentState {

    fn set_content(&mut self, content: String) {
        println!("Setting content to: {}", content);
        self.content = content.clone();
        self.content_reactive.set(content.clone());
        self.content_reactive.value = content;
        println!("Content set to: {}", self.content);
        println!("Content 2 set to: {}", self.content_reactive.value);
    }
}

impl HeaderComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let state_factory = || HeaderComponentState::new(
            String::from("Hello from header-component")
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
                ],
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }
}
    