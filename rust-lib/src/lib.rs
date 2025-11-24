mod serde_structs;
mod sorting_structs;
mod sorting;
use wasm_bindgen::prelude::*;
use std::panic;
use serde_structs::Course;
use sorting_structs::*;
use std::collections::HashMap;
use crate::serde_structs::{Coursedata, ScheduleSelection, make_course_cache};

//TODO: impliment GenEd

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
/// Takes an input of Course[] and json, and returns ScheduleSelection[][] (a list of schedules)
pub fn get_schedules(courses: JsValue, building_data: JsValue) -> JsValue {
    // get courses
    let from_serde: Vec<Course> = match serde_wasm_bindgen::from_value(courses.clone()) {
        Ok(val) => val,
        Err(err) => {
            console_log!("Course data deserialize error: {}", err);
            log_js(&courses);
            Vec::new()
        }
    };
    //format all sections into ScheduleSelection and save for later
    let course_cache: Coursedata = make_course_cache(&from_serde);

    //format all sections for schedule making
    let course_map: CourseMap = from_serde.into_iter().map(|f|f.to_coursemap()).collect();

    //get building location data
    let buildings: HashMap<String, BuildingData> = match serde_wasm_bindgen::from_value(building_data.clone()) {
        Ok(val) => val,
        Err(err) => {
            console_log!("Building data deserialize error: {}", err);
            log_js(&building_data);
            HashMap::new()
        }
    };

    

    //generate all potential schedules
    let potential_schedules: Vec<Schedule> = sorting::get_potential_schedules(course_map, &buildings);

    //convert to ScheduleSelection[][] using the saved cache
    let output: Vec<Vec<ScheduleSelection>> = potential_schedules.iter().map(|schedule|
        schedule.iter().map(|section| course_cache
            .get(&section.course)
            .and_then(|c|c.get(&section.section))
            .cloned()
            .unwrap_or_default()
        ).collect()
    ).collect();

    let js_val = serde_wasm_bindgen::to_value(&output).unwrap_or_default();
    //log_js(&js_val);
    return js_val;
}