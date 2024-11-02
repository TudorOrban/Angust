use tokio::runtime::Runtime;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ASYNC_RUNTIME: Lazy<Mutex<Runtime>> = Lazy::new(|| {
    Mutex::new(tokio::runtime::Runtime::new().unwrap())
});

fn run_async<F, T>(future: F) -> tokio::task::JoinHandle<T>
where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let rt = ASYNC_RUNTIME.lock().unwrap();
    rt.spawn(future)
}