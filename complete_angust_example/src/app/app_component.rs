
use std::collections::HashMap;

use angust::{
    component_state, 
    rendering::elements::component::{component::Component, component_factory::ComponentFactory
}};


pub struct AppComponent {
    component: Component<AppComponentState>,    
}

component_state! {
    AppComponentState {
        content: String,
    }
}

impl AppComponentState {
    // fn new() -> Self {
    //     Self { content: String::from("Hello, App Component!") }
    // }

    pub fn toggle_content(&mut self) {
        if self.content == "Hello, App Component!" {
            self.content = "Hello, World!".to_string();
        } else {
            self.content = "Hello, App Component!".to_string();
        }
    }
}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let state_factory = || AppComponentState::new(String::from("AS"));

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
        println!("{}", state.content);
    }
}
    