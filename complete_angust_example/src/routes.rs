use std::collections::HashMap;

use angust::rendering::router::angust_router::init_global_router;


pub fn register_routes() {
    
    let mut routes = HashMap::new();
    routes.insert(String::from("/header"), String::from("header-component"));
    routes.insert(String::from("/main-menu"), String::from("main-menu-component"));

    init_global_router(routes);
}