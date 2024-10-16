use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};

pub struct ReactiveField<T> {
    value: T,
    listeners: Arc<Mutex<Vec<Box<dyn Fn() + Send + Sync>>>>,
}

impl<T> Debug for ReactiveField<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T> ReactiveField<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.listeners.lock().unwrap().push(Box::new(callback));
    }

    fn notify_listeners(&self) {
        for callback in self.listeners.lock().unwrap().iter() {
            callback();
        }
    }
}

impl<T> Deref for ReactiveField<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for ReactiveField<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.notify_listeners();
        &mut self.value
    }
}
