use crate::app::features::products::models::product::Product;


pub struct ProductService {

}

impl ProductService {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn get_products(&self) -> Vec<Product> {
        let prod1 = Product::new(1, "product1".to_string(), 1);
        let prod2 = Product::new(2, "product2".to_string(), 1);
        let prod3 = Product::new(3, "product3".to_string(), 1);
        
        vec![prod1, prod2, prod3]
    }

}
    