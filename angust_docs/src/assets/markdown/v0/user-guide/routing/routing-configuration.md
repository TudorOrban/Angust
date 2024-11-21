&nbsp;

# Routing Configuration

A lot of applications consist of two base elements: a navigation menu (for instance in a Header or Sidebar), and the main page content, which varies from one navigation item to another. Instead of building this all this functionality manually, you can rely on the Angust Router. For this you need to do two things: *register your routes* and using the *Router Component*.

## Using the router

This involves telling the Angust Router what navigation items your app should have, and to what components they should map. This is done by convention in a `routes.rs` module at the root of `src`, which should already exist if you've generated the project with the CLI tool.

Suppose you want the navigation items `Home`, `Dashboard` and `Settings`, and you have generated a component for each of their corresponding page content. Then you can configure the routes as follows:

```rust
use std::collections::HashMap;

use angust::rendering::router::router_proxy::{init_global_router, RouteConfiguration};

pub fn register_routes() {
    let mut routes = HashMap::new();

    routes.insert("Home".to_string(), "home-component".to_string());
    routes.insert("Dashboard".to_string(), "dashboard-component".to_string());
    routes.insert("Settings".to_string(), "settings-component".to_string());
    
    let route_config = RouteConfiguration {
        routes,
        initial_route: Some("Home".to_string()),
        cache_pages: false,
    };

    init_global_router(route_config);
}
```

Here we map the items to the aliases of the components defined in their corresponding modules. We also tell Angust to start the application with `Home` as the current route.

Now you need to only place the special `router-component` wherever you want your page content to be. For instance, if you have a `header-component` where the navigation items are displayed, your `app-component` might look like this:

```html
<div>
    <header-component></header-component>

    <router-component></router-component>
</div>
```