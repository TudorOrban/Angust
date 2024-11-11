use crate::app::features::products::models::product::Product;


pub struct ProductService {
    products: Vec<Product>,
}

impl ProductService {
    pub fn new() -> Self {
        Self {
            products: vec![],
        }
    }

    pub async fn get_products(&self) -> Vec<Product> {
        // Sleep 1 sec to simulate a network call
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let prod1 = Product::new(1, "product1".to_string(), 1);
        let prod2 = Product::new(2, "product2".to_string(), 1);
        let prod3 = Product::new(3, "product3".to_string(), 1);
        
        vec![prod1, prod2, prod3]
    }

}
    