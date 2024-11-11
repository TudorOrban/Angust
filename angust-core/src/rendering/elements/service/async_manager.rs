
use std::{any::Any, collections::HashMap, fmt, future::Future, sync::{Arc, Mutex}};

use lazy_static::lazy_static;
use tokio::{runtime::Handle, task::JoinHandle};

use crate::application::event_loop_proxy::{get_event_loop_proxy, ApplicationEvent};


/*
 * This function allows the user to return to the GUI thread to handle the response of an async operation.
 * It achieves this by posting A ExecuteTask event through the event loop proxy.
 */
pub fn post_to_gui_thread<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    let event_loop_proxy_opt = get_event_loop_proxy();
    if event_loop_proxy_opt.is_none() {
        eprintln!("Event loop proxy not set");
        return;
    }
    let proxy = event_loop_proxy_opt.unwrap();

    let task = ClosureExecutor::new(f);
    proxy.send_event(ApplicationEvent::ExecuteTask(task))
            .expect("Failed to send event to GUI thread");
}

// Struct that safely encapsulates a FnOnce closure for message passing between threads
pub struct ClosureExecutor {
    closure: Arc<Mutex<Option<Box<dyn FnOnce() + Send>>>>,
}

impl ClosureExecutor {
    pub fn new<F>(closure: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        ClosureExecutor {
            closure: Arc::new(Mutex::new(Some(Box::new(closure)))),
        }
    }

    pub fn execute(&self) {
        let mut closure_opt = self.closure.lock().unwrap();
        if let Some(closure) = closure_opt.take() {
            closure();
        }
    }
}

impl fmt::Debug for ClosureExecutor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClosureExecutor")
    }
}


// This allows the user to chain the async operation with post_to_gui_thread
pub trait FutureExt: Future {
    fn post_to_gui_thread<F>(self, f: F) -> JoinHandle<()>
    where
        F: FnOnce(Self::Output) + Send + 'static,
        Self: Sized + Send + 'static,
        Self::Output: Send;
}

impl<T> FutureExt for T
where
    T: Future + Send + 'static,
    T::Output: Send,
{
    fn post_to_gui_thread<F>(self, f: F) -> JoinHandle<()>
    where
        F: FnOnce(T::Output) + Send + 'static,
    {
        run_async(async move {
            let result = self.await;
            post_to_gui_thread(move || f(result));
        })
    }
}

// Utility function that grabs the current handle to the Tokio runtime (Handle::current()) and spawns the future onto this runtime
pub fn run_async<F, T>(future: F) -> tokio::task::JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let handle = Handle::current();
    handle.spawn(future)
}


type Callback = Box<dyn Fn(&dyn Any) + Send>;

pub struct EventManager {
    callbacks: Mutex<HashMap<usize, Callback>>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            callbacks: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_callback<F>(&self, id: usize, callback: F)
    where
        F: Fn(&dyn Any) + Send + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.insert(id, Box::new(callback));
    }

    pub fn trigger_event(&self, id: usize, data: &dyn Any) {
        if let Some(callback) = self.callbacks.lock().unwrap().get(&id) {
            callback(data);
        }
    }
}

// Initialize the event manager as a global instance
lazy_static! {
    pub static ref EVENT_MANAGER: EventManager = EventManager::new();
}