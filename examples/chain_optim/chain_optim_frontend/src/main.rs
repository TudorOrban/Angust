
extern crate angust;

use angust::application::application::Application;

mod app;
mod component_registration;
mod service_registration;
mod routes;


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
    routes::register_routes();

    let mut app = Application::new(initial_state, String::from("ChainOptim"));
    
    app.run();
}
    
    
    