extern crate angust;

use angust::{application::application::Application, rendering::elements::component::component::Component};

pub struct AppState {
    pub message: String,
}

pub enum Message {
    UpdateMessage(String),
}

pub struct CustomComponent {
    component: Component<CustomComponentState>,    
}

pub struct CustomComponentState {
    content: String,
}

impl CustomComponent {
    pub fn new() -> Self {
        let state = CustomComponentState { content: String::from("Initial Content") };
        let component = Component::new(
            "CustomComponent".to_string(),
            "resources/html/custom_component.html".to_string(),
            state,
        );
        Self { component }
    }

    fn toggle_content(&mut self) {
        if self.component.state.content == "Initial Content" {
            self.component.state.content = String::from("Updated Content");
        } else {
            self.component.state.content = String::from("Initial Content");
        }
    }

}

fn main() {
    let initial_state = AppState {
        message: "Hello, world!".to_string(),
    };

    let mut app = Application::new(initial_state, String::from("New Test App"));
    
    app.run();
}