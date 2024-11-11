use angust_macros::component_state;


#[component_state]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub user_id: u32,
}
