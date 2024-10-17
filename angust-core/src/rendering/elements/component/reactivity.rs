use std::fmt::{Debug, Formatter};


pub struct ReactiveField<T> {
    pub value: T,
    listeners: Vec<Box<dyn FnMut(&ComponentEvent)>>,  
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
            listeners: Vec::new(),
        }
    }

    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: 'static + FnMut(&ComponentEvent),
    {
        self.listeners.push(Box::new(callback));
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
        self.notify_listeners(&ComponentEvent::ReloadTemplate(String::from("")));
    }

    fn notify_listeners(&mut self, event: &ComponentEvent) {
        for callback in self.listeners.iter_mut() {
            callback(event); 
        }
    }
}

pub enum ComponentEvent {
    ReloadTemplate(String),
}

pub struct EventQueue {
    events: Vec<ComponentEvent>,  // Store events
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn push(&mut self, event: ComponentEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<ComponentEvent> {
        std::mem::take(&mut self.events)
    }
}
