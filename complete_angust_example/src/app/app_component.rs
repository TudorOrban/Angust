

use std::collections::HashMap;

use angust::{
    define_component_state, rendering::elements::component::{component::Component, component_factory_registry::ComponentFactory
}};


define_component_state! {
    AppComponentState {
        content: String,
        count: i32,
    }
}


impl AppComponentState {
    
    pub fn toggle_content(&mut self) {
        if self.content.value == "Hello, App Component!" {
            self.content.value = "Hello, World!".to_string();
        } else {
            self.content.value = "Hello, App Component!".to_string();
        }
    }

    pub fn increment_count(&mut self) {
        self.count.value += 1;
    }
}


pub struct AppComponent {
    component: Component<AppComponentState>,    
}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let state_factory = || AppComponentState::new(
            String::from("Hello, App Component!"),
            0
        );

        registry.insert("app-component".to_string(), Box::new(move || {
            let mut component = Component::new(
                "app-component".to_string(),
                "src/app/app_component.html".to_string(),
                state_factory() 
            );

            component.add_event_handler(String::from("print_something"), |state| {
                Self::print_something(state);
            });
            component.add_event_handler(String::from("increment_count"), |state| {
                Self::increment_count(state);
            });


            Box::new(component)
        }));
    }

    fn print_something(state: &mut AppComponentState) {
        state.toggle_content();
        println!("{}", state.content.value);
    }

    fn increment_count(state: &mut AppComponentState) {
        state.increment_count();
    }
}
    