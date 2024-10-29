

use std::{collections::HashMap, any::Any};

use angust::{
    rendering::elements::component::{
        component::Component, 
        component_factory_registry::ComponentFactory, 
        state::{reactivity::{ComponentEvent, ReactiveField, ReactiveState}, reflectivity::ReflectiveState}, 
        functions::component_functions::ComponentFunctions, 
    }, 
    wrap_fn, wrap_fn_mut
};
use angust_macros::component_state;

#[component_state]
#[derive(Clone)]
struct UIItem {
    label: String,
    value: String,
    items: Vec<String>,
}

#[component_state]
#[derive(Clone)]
struct AppComponentState {
    name: String,
    content: String,
    count: f64,
    active_tab: String,
    ui_items: Vec<UIItem>,
    items: Vec<String>,
    item: UIItem,
}

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

    pub fn set_active_tab(&mut self, tab_name: String) {
        self.active_tab_reactive.set(tab_name);
        self.toggle_content();
    }
}

pub struct AppComponent {

}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("app-component".to_string(), Box::new(move || {
            let state_factory = || AppComponentState::new(
                "Alice".to_string(),
                "Hello, App Component!".to_string(),
                0.0,
                "Home".to_string(),
                vec![
                    UIItem::new("Label 1".to_string(), "Value 1".to_string(), 
                    vec![
                        "Item 11".to_string(),
                        "Item 12".to_string(),
                        "Item 13".to_string(),
                    ]),
                    UIItem::new("Label 2".to_string(), "Value 2".to_string(), 
                    vec![
                        "Item 21".to_string(),
                        "Item 22".to_string(),
                    ]),
                ],
                vec![],
                UIItem::new("Label 3".to_string(), "Value 3".to_string(),
                vec![
                    "Item 31".to_string(),
                    "Item 32".to_string(),
                    "Item 33".to_string(),
                ]),
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
                ],
                vec![
                    ("set_active_tab", wrap_fn_mut!(AppComponentState, AppComponentState::set_active_tab, String)),
                ],
                vec![],
            );
            component.add_component_functions(component_functions);

            component.initialize();

            Box::new(component)
        }));
    }

}
    