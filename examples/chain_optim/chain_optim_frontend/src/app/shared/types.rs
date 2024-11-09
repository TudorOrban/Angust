use angust_macros::component_state;


#[component_state]
pub struct UIItem {
    label: String,
    value: String,
}