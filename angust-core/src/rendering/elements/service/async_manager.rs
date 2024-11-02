
use std::{future::Future, sync::{Arc, Mutex}, fmt};

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
    if let Some(proxy) = get_event_loop_proxy() {
        let task = ClosureExecutor::new(f);
        proxy.send_event(ApplicationEvent::ExecuteTask(task))
             .expect("Failed to send event to GUI thread");
    } else {
        eprintln!("Event loop proxy not set");
    }
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