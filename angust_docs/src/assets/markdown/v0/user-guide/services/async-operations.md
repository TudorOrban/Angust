&nbsp;

# Async Operations

Very often, you will want your services to include async functions. This is especially important for operations that call some backend endpoint, which *should not block the main thread* (or *GUI thread*). 

&nbsp;

## Using Async functions

To be able to use Async functions, you need an asynchronous Rust runtime. Currently the only supported runtime is `tokio`, which should suffice for most use cases. 

To use `tokio`, add it to your dependencies:

```toml
[dependencies]
tokio = "1.41.0"
```

and add the `#[tokio::main]` attribute to your main function:

```rust
#[tokio::main]
async fn main() {
    // Application
}
```

If you've generated the project with the CLI tool, this setup should already be in place. Now you can simply add async functions to your services, for instance:

```rust
pub struct ProductService;

impl ProductService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_products_async(&self) -> Vec<Product> {
        sleep(Duration::from_secs(2)); // Simulate an asynchronous delay

        let prod1 = Product::new(1, "Samsung Galaxy S21".to_string());
        let prod2 = Product::new(2, "Samsung Galaxy S22".to_string());

        vec![prod1, prod2]
    }
}
```

## Handling response

In many scenarios like the above, you would usually want to use the response of the operation to update the state of the application. However, to safely achieve this, state updates *should happen on the GUI thread*. 

To this end, we provide the `post_to_gui_thread` function. Here is how you can use it along with the `get_products_async` above:

```rust
use angust::rendering::elements::service::{async_manager::FutureExt, service_registry::get_global_service};

#[component_state]
struct ProductsComponentState {
    products: Vec<Product>,
}

impl ProductsComponentState {
    fn init(&mut self) {
        let product_service: &ProductService = get_global_service("product-service").unwrap();

        product_service.get_products_async()
            .post_to_gui_thread(|products| {
                self.products = products;
            });
    }
}
```

With this approach, the `products` on `ProductsComponentState` will safely get updated with the response of the `get_products_async`.

> **Important Note**: The described pattern currently has a lifetime issue: capturing the reference to the state in a callback. This issue will be fixed as soon as possible, stay tuned for updates.

&nbsp;

## Next Step

By now you should have the main tools needed to build Angust applications. Next you could check out the [Router](https://tudororban.github.io/Angust/v0/user-guide/router) provided by Angust, or delve into some [Best Practices](https://tudororban.github.io/Angust/v0/user-guide/best-practices) for building scalable apps.

&nbsp;