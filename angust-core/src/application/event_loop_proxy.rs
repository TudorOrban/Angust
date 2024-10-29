use once_cell::sync::Lazy;
use std::sync::Mutex;
use winit::event_loop::EventLoopProxy;

use crate::rendering::elements::component::state::reactivity::ComponentEvent;


static GLOBAL_PROXY: Lazy<Mutex<Option<EventLoopProxy<ComponentEvent>>>> = Lazy::new(|| Mutex::new(None));

pub fn set_event_loop_proxy(proxy: EventLoopProxy<ComponentEvent>) {
    let mut global_proxy = GLOBAL_PROXY.lock().unwrap();
    *global_proxy = Some(proxy);
}

pub fn get_event_loop_proxy() -> Option<EventLoopProxy<ComponentEvent>> {
    let global_proxy = GLOBAL_PROXY.lock().unwrap();
    global_proxy.clone()
}
