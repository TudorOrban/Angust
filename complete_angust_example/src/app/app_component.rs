

use std::{collections::HashMap, any::Any};

use angust::{
    define_component_state, 
    rendering::elements::component::{
        component::Component, 
        component_factory_registry::ComponentFactory, 
        functions::component_functions::ComponentFunctions, test::Reflect
    }, wrap_fn
};


struct Address {
    street: String,
    zip: u32,
}

struct User {
    name: String,
    age: u8,
    address: Address,
}

impl Reflect for Address {
    fn get_field(&self, name: &str) -> Option<&dyn Reflect> {
        match name {
            "street" => Some(&self.street),
            "zip" => Some(&self.zip),
            _ => None,
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Reflect for User {
    fn get_field(&self, name: &str) -> Option<&dyn Reflect> {
        match name {
            "name" => Some(&self.name),
            "age" => Some(&self.age),
            "address" => Some(&self.address),
            _ => None,
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
fn print_reflect_value(field: &dyn Reflect) {
    if let Some(val) = field.as_any().downcast_ref::<String>() {
        println!("String value: {}", val);
    } else if let Some(val) = field.as_any().downcast_ref::<u32>() {
        println!("u32 value: {}", val);
    } else if let Some(val) = field.as_any().downcast_ref::<u8>() {
        println!("u8 value: {}", val);
    } else {
        println!("Unknown type");
    }
}
fn get_nested_field<'a>(obj: &'a dyn Reflect, path: &[&str]) -> Option<&'a dyn Reflect> {
    let mut current = obj;
    for &field in path {
        if let Some(next) = current.get_field(field) {
            current = next;
        } else {
            return None;
        }
    }
    Some(current)
}

define_component_state! {
    AppComponentState {
        content: String,
        count: f64,
        active_tab: String,
        // items: Vec<String>,
        some_state_part: SomeStatePart,
    }
}

define_component_state! {
    SomeStatePart {
        value: String,
        is_active: bool,
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
    
    pub fn is_active_tab(&self, tab_name: String, is_active: bool) -> bool {
        if !is_active {  
            return false;
        }

        self.active_tab.value == tab_name
    }

    pub fn set_active_tab_home(&mut self) {
        self.active_tab.set("Home".to_string());
    }

    pub fn set_active_tab_dashboard(&mut self) {
        self.active_tab.set("Dashboard".to_string());
    }

    pub fn set_active_tab_settings(&mut self) {
        self.active_tab.set("Settings".to_string());
    }
}

pub struct AppComponent {
    component: Component<AppComponentState>,    
}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let user = User {
            name: "Alice".to_string(),
            age: 30,
            address: Address {
                street: "123 Main St".to_string(),
                zip: 90210,
            },
        };
    
        
    if let Some(field) = get_nested_field(&user, &["address", "street"]) {
        print_reflect_value(field);
    }

        registry.insert("app-component".to_string(), Box::new(move || {
            let some_state_part = SomeStatePart::new(
                String::from("Some value"),
                true,
            );
            let state_factory = || AppComponentState::new(
                String::from("Hello, App Component!"),
                0.0,
                String::from("Home"),
                // vec![
                //     String::from("Home"),
                //     String::from("Dashboard"),
                //     String::from("Settings"),
                // ],
                some_state_part,
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
    