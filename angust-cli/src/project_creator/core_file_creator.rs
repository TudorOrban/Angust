use std::{fs, path::PathBuf};



pub fn create_core_app_files(project_root_path: &PathBuf, main_rs_path: &String, index_html_path: &String) {
    update_main_rs_file(project_root_path, main_rs_path); // Already created by cargo init
    create_component_registration_file(project_root_path, &String::from("src/component_registration.rs"));
    create_service_registration_file(project_root_path, &String::from("src/service_registration.rs"));
    create_routes_registration_file(project_root_path, &String::from("src/routes.rs"));
    create_index_html_file(project_root_path, index_html_path);
}

fn update_main_rs_file(project_root_path: &PathBuf, main_rs_path: &String) {
    let main_rs_path = project_root_path.join(main_rs_path);

    let main_rs_contents = r#"
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

    let mut app = Application::new(initial_state, String::from("New Angust App"));
    
    app.run();
}
    
    
    "#;

    fs::write(&main_rs_path, main_rs_contents)
        .expect("Failed to write main.rs file");
}

fn create_component_registration_file(project_root_path: &PathBuf, component_registration_path: &String) {
    let component_registration_path = project_root_path.join(component_registration_path);

    let component_registration_contents = r#"
use std::collections::HashMap;

use angust::rendering::elements::component::component_factory_registry::initialize_component_registry;

use crate::app::app_component::AppComponent;

/*
 * Function for registering all user-defined components. Should be called before Application::new()
 */
pub fn register_components() {
    let mut registry = HashMap::new();

    AppComponent::register(&mut registry);

    initialize_component_registry(registry);
}
"#;

    fs::write(&component_registration_path, component_registration_contents)
        .expect("Failed to write component_registration.rs file");
}

fn create_service_registration_file(project_root_path: &PathBuf, service_registration_path: &String) {
    let service_registration_path = project_root_path.join(service_registration_path);

    let service_registration_contents = r#"
use angust::rendering::elements::service::service_registry::{initialize_service_registry, ServiceRegistry};


pub fn register_services() {
    let mut registry = ServiceRegistry::new();

    initialize_service_registry(registry);
}   
    
    "#;

    fs::write(&service_registration_path, service_registration_contents)
        .expect("Failed to write service_registration.rs file");
}

fn create_routes_registration_file(project_root_path: &PathBuf, routes_registration_path: &String) {
    let routes_registration_path = project_root_path.join(routes_registration_path);

    let routes_registration_contents = r#"
use std::collections::HashMap;

use angust::rendering::router::router_proxy::{init_global_router, RouteConfiguration};


pub fn register_routes() {
    let mut routes = HashMap::new();
    
    let route_config = RouteConfiguration {
        routes,
        initial_route: None,
        cache_pages: true,
    };

    init_global_router(route_config);
}

    "#;

    fs::write(&routes_registration_path, routes_registration_contents)
        .expect("Failed to write routes_registration.rs file");
}


fn create_index_html_file(project_root_path: &PathBuf, index_html_path: &String) {
    let index_html_path = project_root_path.join(index_html_path);

    let index_html_contents = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body>
    <div>
        <app-component></app-component>
    </div>
</body>
</html>
"#;

    fs::write(&index_html_path, index_html_contents)
        .expect("Failed to write index.html file");
}