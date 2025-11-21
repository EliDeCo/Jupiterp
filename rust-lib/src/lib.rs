use wasm_bindgen::prelude::*;
use std::panic;

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
pub fn add(a: u32) -> u32 {
    console_log!("Added {}", a);
    a + 1
}

