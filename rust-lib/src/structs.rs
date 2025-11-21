use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct CourseRust {
    pub courseCode: String,
}

#[wasm_bindgen]
impl CourseRust {
    #[wasm_bindgen(constructor)]
    pub fn new(courseCode: String) -> CourseRust {
        CourseRust { courseCode }
    }
}