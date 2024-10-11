extern crate angust;

use angust::application::application::Application;

pub struct AppState {
    pub message: String,
}

pub enum Message {
    UpdateMessage(String),
}

fn main() {
    let initial_state = AppState {
        message: "Cartan Browser!".to_string(),
    };

    let mut app = Application::new(initial_state, String::from("Cartan"));
    
    app.run();
}