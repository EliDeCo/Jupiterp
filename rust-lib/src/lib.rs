/**
 * This file is part of Jupiterp. For terms of use, please see the file
 * called LICENSE at the top level of the Jupiterp source tree (online at
 * https://github.com/atcupps/Jupiterp/LICENSE).
 * Copyright (C) 2024 Andrew Cupps
 */

mod serde_structs;
mod sorting;
mod sorting_structs;
use crate::{
    serde_structs::{Coursedata, ScheduleSelection, make_course_cache},
    sorting::get_potential_schedules,
};
use serde::Serialize;
use serde_structs::Course;
use serde_wasm_bindgen::Serializer;
use sorting_structs::*;
use std::panic;
use wasm_bindgen::prelude::*;

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
/// Takes an input of Course[], and returns ScheduleSelection[][]
pub fn get_schedules(courses: JsValue) -> JsValue {
    let course_list: Vec<Course> = match serde_wasm_bindgen::from_value(courses) {
        Ok(val) => val,
        Err(err) => {
            console_log!("Course data deserialize error: {}", err);
            Vec::new()
        }
    };

    let course_cache: Coursedata = make_course_cache(&course_list);

    let potential_schedules: Vec<Vec<Section>> = get_potential_schedules(course_list);

    //convert to ScheduleSelection[][] using the saved cache
    let output: Vec<Vec<ScheduleSelection>> = potential_schedules
        .into_iter()
        .map(|schedule| {
            schedule
                .into_iter()
                .enumerate()
                .map(|(i, section)| {
                    course_cache
                        .get(&section.course)
                        .and_then(|c| c.get(&section.section))
                        .unwrap_throw()
                        .clone()
                        .set_color(i as i32)
                })
                .collect()
        })
        .collect();

    let js_val: JsValue = output
        .serialize(&Serializer::json_compatible())
        .unwrap_throw();

    return js_val;
}
