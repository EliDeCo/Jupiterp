use crate::{sorting::is_conflict};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//
pub type CourseMap = HashMap<String, SectionMap>;
pub type SectionMap = HashMap<String, Section>;
pub type Classtimes = HashMap<u32, Vec<StartEnd>>;
pub type ClasstimesForHumans = Vec<String>;
pub type BuildingMap = HashMap<String, BuildingData>;
pub type Schedule = Vec<Section>;
pub type ScheduleWithAlternates = Vec<(Section, Vec<Section>)>; // a schedule where each section has a list of alternates

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Section {
    pub professor: ProfData,
    pub classtimes: Classtimes,
    pub course: String,
    pub section: String,
    pub seats: [u32; 3], //Total, open, waitlisted
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct StartEnd {
    pub building: String,
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildingData {
    //pub name: String,
    //pub id: String,
    pub long: f32,
    pub lat: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProfData {
    pub name: String,
    pub rating: f32,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct DisplaySection {
    pub professor: ProfData,
    pub classtimes: ClasstimesForHumans,
    pub course: String,
    pub section: String,
    pub seats: [u32; 3], //Total, open, waitlisted
    pub alternates: String,
}

//this is the type that the professor rating API returns
#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct ProfRatingInput {
    //courses: Vec<String>,
    pub average_rating: f32,
    //type: String,
    //name: String,
    //slug: String,
}

impl PartialEq for Section {
    fn eq(&self, _other: &Self) -> bool {
        self.course == _other.course && self.section == _other.section
    }
}

impl Section {
    #[allow(dead_code)] //functionality for generating alternates may be added in the future
    ///Finds an alternate sections that can replace this section in the given schedule
    pub fn find_alt(
        &self,
        mut schedule: Vec<Section>,
        buildings: &HashMap<String, BuildingData>,
        walk_speed: f32,
        earliest: u32,
        latest: u32,
        alternates: &CourseMap,
    ) -> Vec<Section> {
        //remove the course in question
        schedule.retain(|s| s != self);

        //test every alternate and keep track of the ones that fit properly
        let mut alts: Vec<Section> = Vec::new();
        for (_, alt_section_map) in alternates {
            //for each alternate course
            'section_loop: for (_, alt_section) in alt_section_map {
                //for each section in that alternate course
                for current_section in &schedule {
                    //see if the alternate section conflicts with any other section in the current schedule
                    if is_conflict(
                        current_section,
                        alt_section,
                        buildings,
                        walk_speed,
                        earliest,
                        latest,
                    ) {
                        continue 'section_loop; //if this section conflicts with anything in the schedule, move on to the next section
                    }
                }
                //if we reach here, that means this alternate is compatible with the whole schedule
                alts.push(alt_section.clone());
            }
        }

        return alts;
    }
}