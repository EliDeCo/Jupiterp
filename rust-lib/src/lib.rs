use wasm_bindgen::prelude::*;
use std::panic;
mod structs;
use structs::*;

#[wasm_bindgen]
/// Sets the panic hook so all panics are forwarded to console.error
pub fn error_init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
/// macro for console.log()
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn test(a: Vec<CourseRust>) {
    console_log!("{}", a.iter().map(|c| c.courseCode.as_str() ).collect::<Vec<_>>().join(" "));
}