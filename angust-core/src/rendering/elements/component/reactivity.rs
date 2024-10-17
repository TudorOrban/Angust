use std::{collections::VecDeque, fmt::{Debug, Formatter}};

use super::{component::Component, component_state::ComponentState};

pub struct ReactiveField<T> {
    value: T,
    listeners: Vec<Box<dyn FnMut(&ComponentEvent)>>,  
}

pub enum ComponentEvent {
    ReloadTemplate,
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
        self.notify_listeners(&ComponentEvent::ReloadTemplate);
    }

    fn notify_listeners(&mut self, event: &ComponentEvent) {
        for callback in self.listeners.iter_mut() {
            callback(event); 
        }
    }
}

pub enum Action<State: ComponentState> {
    ReloadComponent(Box<dyn FnOnce(&mut Component<State>)>),
}

pub struct ActionQueue<State: ComponentState> {
    queue: Vec<Action<State>>,
}

impl<State: ComponentState> ActionQueue<State> {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn push(&mut self, action: Action<State>) {
        self.queue.push(action);
    }

    pub fn process(&mut self, component: &mut Component<State>) {
        while let Some(action) = self.queue.pop() {
            match action {
                Action::ReloadComponent(mut f) => f(component),
            }
        }
    }
}
