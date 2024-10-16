pub trait Signal<T> {
    fn connect<F>(&mut self, callback: F) where F: Fn(&T) + 'static;
    fn emit(&self, value: &T);
}

pub struct SignalImpl<T> {
    subscribers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> SignalImpl<T> {
    pub fn new() -> Self {
        SignalImpl {
            subscribers: Vec::new(),
        }
    }
}

impl<T> Signal<T> for SignalImpl<T> {
    fn connect<F>(&mut self, callback: F)
    where
        F: Fn(&T) + 'static,
    {
        self.subscribers.push(Box::new(callback));
    }

    fn emit(&self, value: &T) {
        for subscriber in &self.subscribers {
            subscriber(value);
        }
    }
}
