
use angust::rendering::elements::component::{component::Component, component_factory::register_component};

pub struct HeaderComponent {
    component: Component<HeaderComponentState>,    
}

pub struct HeaderComponentState {
    content: String,
}

impl HeaderComponentState {
    pub fn new() -> HeaderComponentState {
        HeaderComponentState {
            content: "Hello from header-component".to_string(),
        }
    }
}

impl HeaderComponent {
    pub fn register() {
        let state_factory = || HeaderComponentState::new();

        register_component("header-component".to_string(), Box::new(move || {
            Component::new(
                "header-component".to_string(),
                "src/app/core/header_component.html".to_string(),
                state_factory() 
            )
        }));

        println!("Registered header-component");
    }
}
    