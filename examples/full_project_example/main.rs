extern crate reast;

use reast::application::application::Application;

pub struct AppState {
    pub message: String,
}

pub enum Message {
    UpdateMessage(String),
}

fn main() {
    let initial_state = AppState {
        message: "Hello, world!".to_string(),
    };

    let mut app = Application::new(initial_state, String::from("New Test App"));
    
    app.run();
}