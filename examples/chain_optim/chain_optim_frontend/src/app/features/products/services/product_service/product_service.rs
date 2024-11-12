use crate::app::features::products::models::product::Product;


pub struct ProductService {

}

impl ProductService {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn get_products(&self) -> Vec<Product> {
        let prod1 = Product::new(1, "Samsung Galaxy S21".to_string(), 1, "A brand-new entry in the Samsung Galaxy series".to_string(), 1000.0, vec!["phones".to_string()]);
        let prod2 = Product::new(2, "Adobe Photoshop".to_string(), 1, "Adobe Photoshop is a raster graphics editor, image-editing software".to_string(), 200.0, vec!["software".to_string()]);
        let prod3 = Product::new(3, "Google Translate".to_string(), 1, "Google Translate is a free and open-source translation service".to_string(), 10.0, vec!["software".to_string(), "languages".to_string()]);
        
        vec![prod1, prod2, prod3]
    }

}
    