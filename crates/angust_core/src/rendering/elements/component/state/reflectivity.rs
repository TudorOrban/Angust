use std::any::Any;

pub trait ReflectiveState {
    fn get_field(&self, name: &str) -> Option<Box<dyn ReflectiveState>>;
    fn set_field(&mut self, name: &str, value: Box<dyn Any>);
    fn get_all_properties(&self) -> Vec<&str>;
    fn as_any(&self) -> Box<dyn Any>; 
    fn clone_box(&self) -> Box<dyn ReflectiveState>;
}

impl Clone for Box<dyn ReflectiveState> {
    fn clone(&self) -> Box<dyn ReflectiveState> {
        self.clone_box()
    }
}

// Implementations
pub struct NoState;

impl ReflectiveState for NoState {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {}

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl Clone for NoState {
    fn clone(&self) -> Self {
        NoState {}
    }
}

impl ReflectiveState for String {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl ReflectiveState for u32 {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl ReflectiveState for usize {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl ReflectiveState for f64 {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }
    
    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
    
    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl ReflectiveState for bool {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
    
    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl<T> ReflectiveState for Vec<T>
where
    T: ReflectiveState + Clone + 'static,
{
    fn get_field(&self, name: &str) -> Option<Box<dyn ReflectiveState>> {
        if let Ok(index) = name.parse::<usize>() {
            self.get(index).map(|item| item.clone_box())
        } else {
            if name == "len" {
                Some(Box::new(self.len()))
            } else {
                None
            }
        }
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec!["len"]
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl<T> ReflectiveState for Option<T>
where
    T: ReflectiveState + Clone + 'static,
{
    fn get_field(&self, name: &str) -> Option<Box<dyn ReflectiveState>> {
        match self {
            Some(inner) => inner.get_field(name),
            None => None,
        }
    }

    fn set_field(&mut self, name: &str, value: Box<dyn Any>) {
        match self {
            Some(inner) => inner.set_field(name, value),
            None => {}
        }
    }

    fn get_all_properties(&self) -> Vec<&str> {
        match self {
            Some(inner) => inner.get_all_properties(),
            None => vec![],
        }
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}