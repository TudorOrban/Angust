use once_cell::sync::Lazy;
use tokio::task::JoinHandle;
use std::{fmt, future::Future, sync::{Arc, Mutex}};
use winit::event_loop::EventLoopProxy;


static GLOBAL_PROXY: Lazy<Mutex<Option<EventLoopProxy<ApplicationEvent>>>> = Lazy::new(|| Mutex::new(None));

pub fn set_event_loop_proxy(proxy: EventLoopProxy<ApplicationEvent>) {
    let mut global_proxy = GLOBAL_PROXY.lock().unwrap();
    *global_proxy = Some(proxy);
}

pub fn get_event_loop_proxy() -> Option<EventLoopProxy<ApplicationEvent>> {
    let global_proxy = GLOBAL_PROXY.lock().unwrap();
    global_proxy.clone()
}


#[derive(Debug)]
pub enum ApplicationEvent {
    StateChange(String),
    ExecuteTask(ClosureExecutor),
}

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

use tokio::runtime::Handle;

pub fn run_async<F, T>(future: F) -> tokio::task::JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let handle = Handle::current();
    handle.spawn(future)
}


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
