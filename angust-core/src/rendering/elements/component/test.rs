pub trait Reflect {
    fn get_field(&self, name: &str) -> Option<&dyn Reflect>;
    
    fn as_any(&self) -> &dyn std::any::Any;
}


impl Reflect for String {
    fn get_field(&self, _name: &str) -> Option<&dyn Reflect> {
        None
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Reflect for u32 {
    fn get_field(&self, _name: &str) -> Option<&dyn Reflect> {
        None
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Reflect for u8 {
    fn get_field(&self, _name: &str) -> Option<&dyn Reflect> {
        None
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}