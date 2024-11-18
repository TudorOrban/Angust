
&nbsp;

# Lifecycle Hooks

The *Lifecycle* of an Angust Component is the chain of operations that get executed from the component's creation to its destruction. Angust provides a way for the user to execute custom operations at certain points in this process, through *Lifecycle Hooks*.

&nbsp;

## Initialization Hook

The lifecycle stage where you will want to insert custom operations most frequently is the initialization stage. For instance, you may want to fetch some data from your backend:

```rust
#[component_state]
struct ProductsComponentState {
    products: Vec<Product>,
}

impl ProductsComponentState {

    fn init(&mut self) {
        self.products = fetch_projects_from_backend();
    }

    fn fetch_projects_from_backend() {
        // Call backend endpoint
    }
}
```

To ensure the `init` function runs during the initialization phase of the `ProductsComponent`, you can follow a similar approach as with Component Functions and Inputs. Namely, transform `init` with a macro provided by Angust, then pass it to `ComponentFunctions`:

```rust
pub struct ProductsComponent;

impl ProductsComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("products-component".to_string(), Box::new(move || {
            let state = ProductsComponentState::new(vec![]);

            let mut component = Component::new(
                "products-component".to_string(),
                "src/app/features/products/components/products_component/products_component.html".to_string(),
                state
            );

            let component_functions = ComponentFunctions::new(
                vec![], vec![], vec![], vec![], vec![], vec![],
                Some(wrap_init_mut!(ProductsComponentState, ProductsComponentState::init)),
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }
}
```

That's it, your products will be fetched *before* `ProductsComponent` loads its template. 

**Note**: While this example may seem trivial, the init hook is especially powerful once *async* operations become involved, which is generally the case when communicating with the backend. In that scenario, starting your endpoint call as soon as possible can significantly improve the performance of your app, and hence the user experience.

&nbsp;

## Other Hooks

Angust does not yet support other hooks, but some important ones will be added soon. Most notably:
- an `onDestroy` hook that runs before the destruction of the component. This is especially useful for ensuring subscriptions to observables get removed when no longer necessary.
- an `onChanges` hook that runs whenever the inputs of the component chnage. This hook is needed when the final value of an input is not available from the start, eg when it's being fetched from the backend.

&nbsp;

## Next Step
Now that you know how to work with Components, you can dive into the other features provided by Angust. [Services](https://tudororban.github.io/Angust/v0/user-guide/services/overview) are a good next step.

&nbsp;