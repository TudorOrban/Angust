
&nbsp;

# Components

Components are self-contained pieces of UI that you can define and use throughout your Angust application. They not only allow you to isolate and reuse chunks of code, but they also hold state and event handlers that make possible the development of highly dynamic apps. This section aims to cover the main features provided by Components.

## Basics

A Component consists of a **HTML Template** and a corresponding **Rust module**. We will explore these in detail in the next subsections, but for now take a look at the `AppComponent` generated already by the `angust_cli create_project` command:

```rust
use std::collections::HashMap;

use angust::rendering::elements::component::{
    component::Component, 
    component_factory_registry::ComponentFactory, 
};
use angust_macros::component_state;


#[component_state]
struct AppComponentState {
    content: String,
}

impl AppComponentState {

}

pub struct AppComponent {

}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("app-component".to_string(), Box::new(move || {
            let state = AppComponentState::new(
                "app-component works!".to_string(),
            );

            let component = Component::new(
                "app-component".to_string(),
                "src/app/app_component.html".to_string(),
                state
            );

            Box::new(component)
        }));
    }

} 
```