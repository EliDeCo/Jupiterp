use wasm_bindgen::prelude::*;
use serde::Deserialize;

//remember, we aren't sorting here. Atleast, not at this moment


//relte signifies these are transitional types between rust and svelte
#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Default)]
pub struct Course {
    #[serde(rename = "courseCode")]
    pub course_code: String,
    //pub name: String,
    //pub min_credits: u32,
    //pub maxCredits: Option<u32>,
    //pub gen_eds: Option<Vec<GenEd>>,
    //pub conditions: Option<Vec<String>>,
    //pub description: Option<String>,
    pub sections: Option<Vec<CourseSection>>
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Default)]
pub struct CourseSection {
    #[serde(rename = "courseCode")]
    pub course_code: String,
    #[serde(rename = "sectionCode")]
    pub section_code: String,
    pub instructors: Vec<String>,
    pub meetings: Vec<ClassMeeting>,
    #[serde(rename = "openSeats")]
    pub open_seats: u32,
    #[serde(rename = "totalSeats")]
    pub total_seats: u32,
    pub waitlist: u32,
    //holdfile: Option<u32>
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Default)]
pub struct ClassMeeting {
    pub classtime: Classtime,
    pub location: Location,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Default)]
pub struct Classtime {
    pub days: String,
    pub start: f32,
    pub end: f32,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Default)]
pub struct Location {
    pub building: String,
    //room: Option<String>,
}