
extern crate angust;

use angust::application::application::Application;

pub mod app;
pub mod component_registration;
pub mod service_registration;


pub struct AppGlobalState {
    pub message: String,
}

#[derive(Clone, Debug)]
struct Product {
    pub id: i32,
    pub name: String,
}

struct ProductService {
    products: Vec<Product>,
}

impl ProductService {
    pub fn new() -> Self {
        ProductService {
            products: vec![
                Product { id: 1, name: "Product 1".to_string() },
                Product { id: 2, name: "Product 2".to_string() },
                Product { id: 3, name: "Product 3".to_string() },
            ]
        }
    }

    pub fn get_products(&self) -> Vec<Product> {
        self.products.clone()
    }
}



fn main() {
    let initial_state = AppGlobalState {
        message: "Hello, Angust user!".to_string(),
    };

    component_registration::register_components();    
    service_registration::register_services();

    let mut app = Application::new(initial_state, String::from("New Angust App"));
    
    app.run();
}
    
    