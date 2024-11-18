
&nbsp;

# Registration and Usage

This section will show you how to register your services with Angust's Service Registry and use them throughout your application.

## Registration

Consider the following example service, which mocks a call to the backend fetching some products:

```rust
use crate::app::features::products::models::product::Product;

pub struct ProductService;

impl ProductService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_products(&self) -> Vec<Product> {
        let prod1 = Product::new(1, "Samsung Galaxy S21".to_string());
        let prod2 = Product::new(2, "Samsung Galaxy S22".to_string());
        
        vec![prod1, prod2]
    }
}
```

To register this service, you need to use Angust's `ServiceRegistry`. This is done by convention in the `service_registration` module at the root of your project, which should already exist if you've generated the project with the CLI tool:

```rust
use crate::app::features::products::services::product_service::product_service::ProductService;

use angust::rendering::elements::service::service_registry::{initialize_service_registry, ServiceRegistry};

pub fn register_services() {
    let mut registry = ServiceRegistry::new();
    registry.add_service("product-service", ProductService::new());

    initialize_service_registry(registry);
}   
```

Then you would call `register_services` during app initialization, similar to component registration.

&nbsp;

## Usage

Once your service is registered, you can directly access it wherever you need it, using the `get_global_service` function. For example:

```rust
use angust::rendering::elements::service::service_registry::get_global_service;

#[component_state]
struct ProductsComponentState {
    products: Vec<Product>,
}

impl ProductsComponentState {
    fn init(&mut self) {
        let product_service: &ProductService = get_global_service("product-service").unwrap();

        self.products = product_service.get_products();
    }
}
```

That's it! The `ProductService` will now fetch the products and put them on `ProductsComponentState`.

**Note**: All Services are currently global, unique and persistent throughout the application's lifetime. We will soon support *Transient* services as well, which can improve performance in certain scenarios.

&nbsp;

## Automatic Generation

As with Components, there is no need to go through the above setup each time you need a new Service. You can generate it automatically with the CLI command:

```
angust_cli generate service src/app/Some
```

This will generate `SomeService` and update the `service_registration` accordingly. Just ensure to provide the path starting with `src/app`, and to provide the service name without the `Service` suffix, which gets added automatically.

&nbsp;

## Next Step

Next, learn how to use Service [async operations](https://tudororban.github.io/Angust/v0/user-guide/services/async-operations).

&nbsp;