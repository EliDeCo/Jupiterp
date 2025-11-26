use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Section {
    pub classtimes: [u64; 5],
    pub course: String,
    pub section: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct StartEnd {
    pub building: String,
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct BuildingData {
    pub long: f32,
    pub lat: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProfData {
    pub name: String,
    pub rating: f32,
}

impl PartialEq for Section {
    fn eq(&self, _other: &Self) -> bool {
        self.course == _other.course && self.section == _other.section
    }
}