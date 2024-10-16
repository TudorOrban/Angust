

use std::collections::HashMap;

use angust::{
    define_component_state, rendering::elements::component::{component::Component, component_factory_registry::ComponentFactory
}};


define_component_state! {
    AppComponentState {
        content: String,
    }
}

impl AppComponentState {
    
    pub fn toggle_content(&mut self) {
        if *self.content == "Hello, App Component!" {
            *self.content = "Hello, World!".to_string();
        } else {
            *self.content = "Hello, App Component!".to_string();
        }
    }
}

pub struct AppComponent {
    component: Component<AppComponentState>,    
}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let state_factory = || {
            let mut state = AppComponentState::new(String::from("Hello, App Component!"));

            // Subscribe to changes on the `content` field
            state.content.subscribe(|| {
                println!("Content field changed!");
            });

            state
        };
        
        registry.insert("app-component".to_string(), Box::new(move || {
            let mut component = Component::new(
                "app-component".to_string(),
                "src/app/app_component.html".to_string(),
                state_factory() 
            );

            component.add_event_handler(String::from("print_something"), |state| {
                Self::print_something(state);
            });

            Box::new(component)
        }));
    }

    fn print_something(state: &mut AppComponentState) {
        state.toggle_content();
        println!("{}", *state.content);
    }
}
    