

use std::{collections::HashMap, any::Any};

use angust::{
    rendering::elements::component::{
        component::Component, 
        component_factory_registry::ComponentFactory, 
        functions::component_functions::ComponentFunctions, 
    }, 
    wrap_fn, wrap_fn_mut, wrap_init_mut
};
use angust_macros::component_state;


#[component_state]
#[derive(Clone)]
struct UIItem {
    label: String,
    value: String,
}

#[component_state]
#[derive(Clone)]
struct AppComponentState {
    content: String,
    active_tab: String,
    ui_items: Vec<UIItem>,
}

impl AppComponentState {

    pub fn init(&mut self) {
        // Init
    }
    
    pub fn is_active_tab(&self, tab_name: String) -> bool {
        println!("Tab name: {:?}", tab_name);
        println!("Active Tab reactive: {:?}", self.active_tab_reactive.value);
        self.active_tab_reactive.value == tab_name
    }

    pub fn set_active_tab(&mut self, tab_name: String) {
        println!("Set Tab name: {:?}", tab_name);
        println!("Set Active Tab reactive: {:?}", self.active_tab_reactive.value);
        self.active_tab_reactive.set(tab_name);
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
                vec![
                    UIItem::new("Home".to_string(), "home".to_string()),
                    UIItem::new("Dashboard".to_string(), "dashboard".to_string()),
                    UIItem::new("Browse".to_string(), "browse".to_string()),
                    UIItem::new("Settings".to_string(), "settings".to_string()), 
                ],
            );

            let mut component = Component::new(
                "app-component".to_string(),
                "src/app/app_component.html".to_string(),
                state_factory() 
            );

            
            let component_functions: ComponentFunctions<AppComponentState> = ComponentFunctions::new(
                vec![],
                vec![],
                vec![],
                vec![
                    ("is_active_tab", wrap_fn!(AppComponentState, AppComponentState::is_active_tab, String)),
                ],
                vec![
                    ("set_active_tab", wrap_fn_mut!(AppComponentState, AppComponentState::set_active_tab, String)),
                ],
                vec![],
                Some(wrap_init_mut!(AppComponentState, AppComponentState::init)),
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }

}
    