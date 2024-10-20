

use std::{collections::HashMap, any::Any};

use angust::{
    rendering::elements::component::{
        component::Component, 
        component_factory_registry::ComponentFactory, 
        functions::component_functions::ComponentFunctions, 
        component_state::{Reflect, ReactiveState},
        reactivity::{ComponentEvent, ReactiveField}
    }, wrap_fn
};
use angust_macros::component_state;

#[component_state]
#[derive(Clone)]
struct Location {
    lat: f64,
    lon: f64,
}

#[component_state]
#[derive(Clone)]
struct Address {
    street: String,
    zip: u32,
    location: Location,
}

#[component_state]
#[derive(Clone)]
struct AppComponentState {
    name: String,
    age: u8,
    address: Address,
    content: String,
    count: f64,
    active_tab: String,
}

// fn print_reflect_value(field: &dyn Reflect) {
//     if let Some(val) = field.as_any().downcast_ref::<String>() {
//         println!("String value: {}", val);
//     } else if let Some(val) = field.as_any().downcast_ref::<u32>() {
//         println!("u32 value: {}", val);
//     } else if let Some(val) = field.as_any().downcast_ref::<u8>() {
//         println!("u8 value: {}", val);
//     } else if let Some(val) = field.as_any().downcast_ref::<f64>() {
//         println!("f64 value: {}", val);
//     } else {
//         println!("Unknown type");
//     }
// }
// fn get_nested_field<'a>(obj: &'a dyn Reflect, path: &[&str]) -> Option<&'a dyn Reflect> {
//     let mut current = obj;
//     for &field in path {
//         if let Some(next) = current.get_field(field) {
//             current = next;
//         } else {
//             return None;
//         }
//     }
//     Some(current)
// }

// define_component_state! {
//     AppComponentState {
//         content: String,
//         count: f64,
//         active_tab: String,
//         // items: Vec<String>,
//         some_state_part: SomeStatePart,
//     }
// }

// define_component_state! {
//     SomeStatePart {
//         value: String,
//         is_active: bool,
//     }
// }

impl AppComponentState {
    
    pub fn toggle_content(&mut self) {
        if self.content == "Hello, App Component!" {
            self.content_reactive.set("Hello, World!".to_string());
        } else {
            self.content_reactive.set("Hello, App Component!".to_string());
        }
    }

    pub fn increment_count(&mut self) {
        let current_value = self.count + 1.0;
        self.count_reactive.set(current_value);
    }
    
    pub fn is_active_tab(&self, tab_name: String, is_active: bool) -> bool {
        if !is_active {  
            return false;
        }

        self.active_tab_reactive.value == tab_name
    }

    pub fn set_active_tab_home(&mut self) {
        self.active_tab_reactive.set("Home".to_string());
    }

    pub fn set_active_tab_dashboard(&mut self) {
        self.active_tab_reactive.set("Dashboard".to_string());
    }

    pub fn set_active_tab_settings(&mut self) {
        self.active_tab_reactive.set("Settings".to_string());
    }
}

pub struct AppComponent {
    _component: Component<AppComponentState>,    
}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        // let state = 
        
        // if let Some(field) = get_nested_field(&state, &["address", "location", "lat"]) {
        //     print_reflect_value(field);
        // }

        registry.insert("app-component".to_string(), Box::new(move || {
            let state_factory = || AppComponentState::new(
                "Alice".to_string(),
                30,
                Address::new(
                    "123 Main St".to_string(),
                    90210,
                    Location::new(
                        34.0522,
                        -118.2437,
                    ),
                ),
                "Hello, App Component!".to_string(),
                0.0,
                "Home".to_string(),
            );

            let mut component = Component::new(
                "app-component".to_string(),
                "src/app/app_component.html".to_string(),
                state_factory() 
            );

            let component_functions: ComponentFunctions<AppComponentState> = ComponentFunctions::new(
                vec![
                    ("print_something", Box::new(|state: &mut AppComponentState| state.toggle_content())),
                    ("increment_count", Box::new(|state: &mut AppComponentState| state.increment_count())),
                ],
                vec![],
                vec![],
                vec![
                    ("is_active_tab", wrap_fn!(AppComponentState, AppComponentState::is_active_tab, String, bool)),
                ]
            );
            component.add_component_functions(component_functions);

            component.initialize();

            Box::new(component)
        }));
    }

}
    