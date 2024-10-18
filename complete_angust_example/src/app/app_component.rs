

use std::collections::HashMap;

use angust::{
    define_component_state, 
    rendering::elements::component::{
        component::Component, 
        component_factory_registry::ComponentFactory, 
        functions::component_functions::ComponentFunctions
    }, wrap_fn
};


define_component_state! {
    AppComponentState {
        content: String,
        count: f64,
        active_tab: String
    }
}

impl AppComponentState {
    
    pub fn toggle_content(&mut self) {
        if self.content.value == "Hello, App Component!" {
            self.content.set("Hello, World!".to_string());
        } else {
            self.content.set("Hello, App Component!".to_string());
        }
    }

    pub fn increment_count(&mut self) {
        let current_value = self.count.value + 1.0;
        self.count.set(current_value);
    }
    
    pub fn is_active_tab(&self, tab_name: String, is_active: bool, some: bool) -> bool {
        if !is_active {  
            return false;
        }

        self.active_tab.value == tab_name
    }

    pub fn get_something(&self, some_param: String) -> String {
        format!("Something: {}", some_param)
    }

    pub fn get_number_plus_one(&self, number: f64) -> f64 {
        number + 1.0
    }
}

pub struct AppComponent {
    component: Component<AppComponentState>,    
}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let state_factory = || AppComponentState::new(
            String::from("Hello, App Component!"),
            0.0,
            String::from("app-component")
        );

        registry.insert("app-component".to_string(), Box::new(move || {
            let mut component = Component::new(
                "app-component".to_string(),
                "src/app/app_component.html".to_string(),
                state_factory() 
            );

            let component_functions: ComponentFunctions<AppComponentState> = ComponentFunctions::new(
                vec![
                    ("print_something", Box::new(|state: &mut AppComponentState| state.toggle_content())),
                    ("increment_count", Box::new(|state: &mut AppComponentState| state.increment_count()))
                ],
                vec![],
                vec![],
                vec![
                    ("is_active_tab", wrap_fn!(AppComponentState, AppComponentState::is_active_tab, String, bool, bool)),
                    ("get_something", wrap_fn!(AppComponentState, AppComponentState::get_something, String)),
                    ("get_number_plus_one", wrap_fn!(AppComponentState, AppComponentState::get_number_plus_one, f64))
                ]
            );
            component.add_component_functions(component_functions);

            component.initialize();

            Box::new(component)
        }));
    }

}
    