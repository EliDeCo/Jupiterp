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
/// Takes an input of Course[] and returns a list of viable schedules
pub fn get_schedules(val: JsValue) {
    let from_serde: Vec<Course> = serde_wasm_bindgen::from_value(val).unwrap_or_default();
    for course in from_serde {
        let section = course.sections.unwrap_or_default()[0].clone();
        console_log!("{}: {} with {}, first meeting is {}-{} in {}. {} open seats",
            course.course_code,
            section.section_code,
            section.instructors.join(" & "),
            section.meetings[0].classtime.start,
            section.meetings[0].classtime.end,
            section.meetings[0].location.building,
            section.open_seats,
        );
    }
}