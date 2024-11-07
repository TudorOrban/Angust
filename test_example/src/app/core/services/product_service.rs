
#[derive(Clone, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
}

pub struct ProductService {
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

    #[allow(dead_code)]
    pub fn get_products(&self) -> Vec<Product> {
        self.products.clone()
    }
    
    pub async fn fetch_products_async(&self) -> Vec<Product> {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        self.products.clone()  // Simulating a fetch operation
    }
}
