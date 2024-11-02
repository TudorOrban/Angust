
use std::{collections::HashMap, any::Any};

use angust::{rendering::elements::component::{
    component::Component, component_factory_registry::ComponentFactory, functions::component_functions::ComponentFunctions, state::{
        reactivity::{ComponentEvent, ReactiveField, ReactiveState}, 
        reflectivity::ReflectiveState
    }
}, wrap_fn_mut};
use angust_macros::component_state;



#[component_state]
pub struct MainMenuComponentState {
    content: String,
}

impl MainMenuComponentState {

    fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

pub struct MainMenuComponent {
    
}

impl MainMenuComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let state_factory = || MainMenuComponentState::new(
            String::from("Hello from main-menu-component")
        );

        registry.insert("main-menu-component".to_string(), Box::new(move || {
            let mut component = Component::new(
                "main-menu-component".to_string(),
                "src/app/core/main_menu_component/main_menu_component.html".to_string(),
                state_factory() 
            );

            let component_functions: ComponentFunctions<MainMenuComponentState> = ComponentFunctions::new(
                vec![], vec![], vec![], vec![], vec![],
                vec![
                    ("set_content", wrap_fn_mut!(MainMenuComponentState, MainMenuComponentState::set_content, String)),
                ],
                None
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }
}
    