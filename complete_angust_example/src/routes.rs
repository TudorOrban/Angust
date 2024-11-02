use std::collections::HashMap;

use angust::rendering::router::angust_router::init_global_router;


pub fn register_routes() {
    
    let mut routes = HashMap::new();
    routes.insert(String::from("/home"), String::from("HomeComponent"));
    routes.insert(String::from("/settings"), String::from("SettingsComponent"));

    init_global_router(routes);
}