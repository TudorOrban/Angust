use std::{fmt::{Debug, Formatter}, ops::{Deref, DerefMut}};

use super::reflectivity::{NoState, ReflectiveState};


pub trait ReactiveState : ReflectiveState {
    fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
        where
            F: 'static + FnMut(&ComponentEvent);
}

impl ReactiveState for NoState {
    fn subscribe_to_property<F>(&mut self, _property_name: &str, _callback: F)
    where
        F: 'static + FnMut(&ComponentEvent),
    {
    }
}

pub struct ReactiveField<T> {
    pub value: T,
    listeners: Vec<Box<dyn FnMut(&ComponentEvent)>>,
}

#[derive(Debug)]
pub enum ComponentEvent {
    StateChange(String),
}

impl<T> Debug for ReactiveField<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T: Clone> Clone for ReactiveField<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            listeners: Vec::new(),
        }
    }
}

impl<T> ReactiveField<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            listeners: Vec::new(),
        }
    }

    // Subscribe to property changes
    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: 'static + FnMut(&ComponentEvent),
    {
        self.listeners.push(Box::new(callback));
    }

    pub fn set(&mut self, new_value: T) {
        self.value = new_value;
        self.notify_listeners(&ComponentEvent::StateChange(String::from("placeholder_id")));
    }

    fn notify_listeners(&mut self, event: &ComponentEvent) {
        for callback in self.listeners.iter_mut() {
            callback(event);
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
        &mut self.value
    }
}


pub struct EventQueue {
    events: Vec<ComponentEvent>,
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
