use std::sync::Mutex;

use once_cell::sync::Lazy;
use winit::event_loop::EventLoopProxy;

use crate::rendering::elements::service::async_manager::ClosureExecutor;


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
