use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, EventTarget};

#[allow(unused_unsafe)]
//this is for the benefit or rust-analyzer, who marks all usages of the regular log_1 as unsafe
pub fn log_1(data_1: &::wasm_bindgen::JsValue) {
    unsafe {
        console::log_1(data_1);
    }
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log_1(&format!($($t)*).into()))
}

pub fn add_callback<T, F>(target: &T, event: &str, f: F) -> Closure<dyn Fn(web_sys::Event)>
where
    T: JsCast,
    F: Fn(web_sys::Event) + 'static,
{
    let closure = Closure::wrap(Box::new(f) as Box<dyn Fn(web_sys::Event)>);

    let target = target
        .dyn_ref::<EventTarget>()
        .expect("target should be an EventTarget");

    target
        .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
        .expect("callback registration should work");
    closure
}
