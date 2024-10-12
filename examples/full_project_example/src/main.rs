extern crate angust;

use angust::{application::application::Application, rendering::elements::component::component::Component};


pub struct AppState {
    pub message: String,
}

pub enum Message {
    UpdateMessage(String),
}

pub struct AppComponent {
    component: Component<AppComponentState>,    
}

pub struct AppComponentState {
    content: String,
}

impl AppComponent {
    pub fn new() -> Self {
        let state = AppComponentState { content: String::from("Hello, App Component!") };

        let mut component = Component::new(
            "AppComponent".to_string(),
            "src/app/app.component.html".to_string(),
            state,
        );

        component.add_event_handler("toggle".to_string(), Box::new(|state: &mut AppComponentState| {
            Self::toggle_content(state);
        }));
        component.add_event_handler("delete".to_string(), Box::new(|state: &mut AppComponentState| {
            Self::delete_content(state);
        }));

        Self { component }
    }

    
    pub fn toggle_content(state: &mut AppComponentState) {
        if state.content == "Initial Content" {
            state.content = String::from("Updated Content");
            println!("Content updated: {}", state.content);
        } else {
            state.content = String::from("Initial Content");
        }
    }

    pub fn delete_content(state: &mut AppComponentState) {
        state.content = String::from("");
    }

}

fn main() {
    let initial_state = AppState {
        message: "Hello, world!".to_string(),
    };

    let mut app = Application::new(initial_state, String::from("New Test App"));
    
    app.run();
}