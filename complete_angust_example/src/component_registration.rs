
use crate::app::app_component::AppComponent;
use crate::app::core::header_component::header_component::HeaderComponent;

/*
 * Function for registering all user-defined components. Should be called before Application::new()
 */
pub fn register_components() {
    AppComponent::register();    

    HeaderComponent::register();
}
