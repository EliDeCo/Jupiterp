mod serde_structs;
mod sorting_structs;
mod sorting;
use wasm_bindgen::prelude::*;
use std::panic;
use serde_structs::Course;
use sorting_structs::*;
//use sorting::{get_potential_schedules, schedules_for_display, schedules_with_alternatives};

//TODO: Impliment Professor Ratings

#[wasm_bindgen]
/// Sets the panic hook so all panics are forwarded to console.error
pub fn error_init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_str(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_js(s: &JsValue);
}

#[allow(unused_macros)]
/// macro for console.log()
macro_rules! console_log {
    ($($t:tt)*) => (log_str(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
/// Takes an input of Course[] and returns a list of viable schedules
pub fn get_schedules(val: JsValue) {
    let from_serde: Vec<Course>;
    match serde_wasm_bindgen::from_value(val.clone()) {
        Ok(v) => { from_serde = v; }
        Err(e) => {
            from_serde = Vec::new();
            console_log!("deserialize error: {}", e);
            log_js(&val);
        }
    }
    let course_map: CourseMap = from_serde.into_iter().map(|f|f.to_coursemap()).collect();
    if course_map.is_empty() {
        console_log!("Coursemap is empty!")
    } else {
        let js_val = serde_wasm_bindgen::to_value(&course_map).unwrap();
        log_js(&js_val);
    }
}