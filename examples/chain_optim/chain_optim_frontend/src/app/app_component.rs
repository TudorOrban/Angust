
use std::collections::HashMap;

use angust::{
    rendering::{elements::component::{
        component::Component, 
        component_factory_registry::ComponentFactory, 
        functions::component_functions::ComponentFunctions, 
    }, router::router_proxy::get_router},
    wrap_fn_mut
};
use angust_macros::component_state;


#[component_state]
struct AppComponentState {
    navigation_items: Vec<String>,
    active_item: String,
}

impl AppComponentState {

    fn navigate_to(&mut self, item: String) {
        let router = get_router();
        router.navigate_to(&item);   
    }
}

pub struct AppComponent;

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("app-component".to_string(), Box::new(move || {
            let state_factory = || AppComponentState::new(
                vec!["Home".to_string(), "Dashboard".to_string(), "Products".to_string(), "Settings".to_string()],
                "Home".to_string(),
            );

            let mut component = Component::new(
                "app-component".to_string(),
                "src/app/app_component.html".to_string(),
                state_factory() 
            );

            let component_functions: ComponentFunctions<AppComponentState> = ComponentFunctions::new(
                vec![], vec![], vec![], 
                vec![], 
                vec![
                    ("navigate_to", wrap_fn_mut!(AppComponentState, AppComponentState::navigate_to, String))
                ], 
                vec![],
                None
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }

}  
