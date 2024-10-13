
 use angust::rendering::elements::component::{component::Component, component_factory::register_component};


pub struct AppComponent {
    component: Component<AppComponentState>,    
}

#[derive(Clone)]
pub struct AppComponentState {
    content: String,
}

impl AppComponentState {
    fn new() -> Self {
        Self { content: String::from("Hello, App Component!") }
    }
}

impl AppComponent {
    pub fn register() {
        let state_factory = || AppComponentState::new();

        register_component("app-component".to_string(), Box::new(move || {
            Component::new(
                "app-component".to_string(),
                "src/app/app_component.html".to_string(),
                state_factory() 
            )
        }));
    }
}
    