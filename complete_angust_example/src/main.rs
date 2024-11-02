
extern crate angust;

use angust::application::application::Application;

pub mod app;
pub mod component_registration;
pub mod service_registration;


pub struct AppGlobalState {
    pub message: String,
}

#[tokio::main]
async fn main() {
    let initial_state = AppGlobalState {
        message: "Hello, Angust user!".to_string(),
    };

    component_registration::register_components();    
    service_registration::register_services();

    let mut app = Application::new(initial_state, String::from("New Angust App"));
    
    app.run();
}
    
    