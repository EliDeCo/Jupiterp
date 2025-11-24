use serde::{Deserialize, Serialize};
use crate::{log_str, sorting_structs::*};
use std::collections::HashMap;

//types for storing course and section data in the correct format for sending to svelte
pub type Coursedata = HashMap<String,SectionData>;
pub type SectionData = HashMap<String,ScheduleSelection>;

#[derive(Deserialize)]
pub struct Course {
    #[serde(rename = "courseCode")]
    pub course_code: String,
    pub name: String,
    #[serde(rename = "minCredits")]
    pub min_credits: u32,
    #[serde(rename = "maxCredits")]
    pub max_credits: Option<u32>,
    #[serde(rename = "genEds")]
    gen_eds: Option<Vec<GenEd>>,
    pub conditions: Option<Vec<String>>,
    pub description: Option<String>,
    pub sections: Option<Vec<CourseSection>>
}


impl Course {
    pub fn to_coursemap(self) -> (String, SectionMap) {
        let mut section_map: SectionMap = HashMap::new();
        if let Some(sections) = self.sections {
            for sec in sections {
                let professor: ProfData = ProfData { name: sec.instructors[0].to_owned(), rating: 0.0 };
                let classtimes: Classtimes = classmeet_convert(sec.meetings);
                let course: String = sec.course_code;
                let section: String = sec.section_code.clone();
                let seats: [u32; 3] = [sec.total_seats, sec.open_seats, sec.waitlist];

                section_map.insert(sec.section_code, Section { professor, classtimes, course, section, seats });
            }
            return (self.course_code, section_map);
        } else {
            log_str(&format!("No courses sections for {}", self.course_code));
            return (self.course_code, HashMap::new());
        }
    }
}


#[derive(Deserialize, Serialize, Default, Clone)]
pub struct CourseSection {
    #[serde(rename = "courseCode")]
    pub course_code: String,
    #[serde(rename = "sectionCode")]
    pub section_code: String,
    pub instructors: Vec<String>,
    pub meetings: Vec<ClassMeetingFull>,
    #[serde(rename = "openSeats")]
    pub open_seats: u32,
    #[serde(rename = "totalSeats")]
    pub total_seats: u32,
    pub waitlist: u32,
    holdfile: Option<u32>
}


#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ClassMeetingFull {
    //normal classes
    Detailed {
        classtime: Classtime,
        location: Location,
    },
    //OnlineAsycn, TBA, etc.
    #[allow(dead_code)]
    Text(String)
}

impl Default for ClassMeetingFull {
    fn default() -> Self {
        ClassMeetingFull::Text(String::from("TBA"))
    }
}

#[derive(Deserialize)]
pub struct ClassMeeting {
    classtime: Classtime,
    location: Location,
}


#[derive(Deserialize, Clone, Default, Serialize)]
pub struct Classtime {
    pub days: String,
    pub start: f32,
    pub end: f32,
}

#[derive(Deserialize, Default, Serialize, Clone)]
pub struct Location {
    pub building: String,
    //room: Option<String>,
}

fn classmeet_convert(input: Vec<ClassMeetingFull>) -> Classtimes {
    let mut output: Classtimes = HashMap::new();
    //iterate through every meeting
    for meeting_full in input {
        let meeting: ClassMeeting = match meeting_full {
            ClassMeetingFull::Detailed { classtime, location } => {
                ClassMeeting { classtime, location }
            }
            ClassMeetingFull::Text(t) => {
                //store abnormal class meetings in day 0, reusing the building field of StartEnd to store which type it is
                let abnormal: StartEnd = StartEnd { 
                    building: t, 
                    start: 0, 
                    end: 0 
                };
                if let Some(day_meetings) = output.get_mut(&0) {
                    day_meetings.push(abnormal);
                } else {
                    output.insert(0, vec![abnormal]);
                }
                continue;

            }
        };


        let building: String = meeting.location.building;

        //turn M into 1, Tu into 2, etc.
        let days_key: Vec<&str> = vec!["M","Tu","W","Th","F"];
        let mut days: Vec<u32> = Vec::new();
        for (i, k) in days_key.iter().enumerate() { 
            if meeting.classtime.days.contains(k) {
                days.push(i as u32 + 1);
            }
        }

        //add format start and end times for each day
        let start_end: StartEnd = StartEnd { 
            building, 
            start: time_fmt(meeting.classtime.start), 
            end: time_fmt(meeting.classtime.end)
        };
        
        //add this meeting to the output, adding the day if it doesn't already exist
        for day in days {
            if let Some(day_meetings) = output.get_mut(&day) {
                day_meetings.push(start_end.clone());
            } else {
                output.insert(day, vec![start_end.clone()]);
            }
        }
    }


    return output;
}

///converts from Jupiterp's way of storing time to the rust scheduling program's way
fn time_fmt(time: f32) -> u32 {
    let hours = time.floor() as u32;
    let minutes = ((time - hours as f32) * 60.0).round() as u32;
    hours * 100 + minutes
}

#[derive(Serialize, Clone, Default)]
pub struct ScheduleSelection {
    pub course: CourseBasic,
    pub section: CourseSection,
    pub hover: bool,
    pub differences: SelectionDifferences,
    #[serde(rename = "colorNumber")]
    pub color_number: i32,   
}

#[derive(Serialize, Clone, Default)]
pub struct CourseBasic {
    #[serde(rename = "courseCode")]
    pub course_code: String,
    name: String,
    #[serde(rename = "minCredits")]
    pub min_credits: u32,
    #[serde(rename = "maxCredits")]
    pub max_credits: Option<u32>,
    #[serde(rename = "genEds")]
    gen_eds: Option<Vec<GenEd>>,
    conditions: Option<Vec<String>>,
    description: Option<String>,
}

#[derive(Serialize, Clone, Default)]
pub struct SelectionDifferences {
    pub instructors: bool,
    #[serde(rename = "numMeetings")]
    pub num_meetings: bool,
    #[serde(rename = "meetingType")]
    pub meeting_type: bool,
    #[serde(rename = "meetingTime")]
    pub meeting_time: bool,
    #[serde(rename = "meetingLocation")]
    pub meeting_location: bool,
}

impl SelectionDifferences {
    pub fn all_false() -> Self {
        Self { 
            instructors: false, 
            num_meetings: false, 
            meeting_type: false, 
            meeting_time: false, 
            meeting_location: false 
        }
    }
}


#[derive(Serialize, Clone, Deserialize)]
pub struct GenEd {
    pub code: String,
    pub name: String,
}

///Makes a cache of all the usefull data from the retrieved courses and sections
pub fn make_course_cache(courses_raw: &Vec<Course>) -> Coursedata {
    return courses_raw.into_iter().map(|course|
        (course.course_code.clone(), course.sections.clone().unwrap_or_default().into_iter().enumerate().map(|(i,section)|{
            (section.section_code.clone(), ScheduleSelection {
                course: CourseBasic { 
                    course_code: course.course_code.clone(), 
                    name: course.name.clone(), 
                    min_credits: course.min_credits, 
                    max_credits: course.max_credits,
                    gen_eds: course.gen_eds.clone(),
                    conditions: course.conditions.clone(), 
                    description: course.description.clone() 
                },
                section: CourseSection { 
                    course_code: course.course_code.clone(), 
                    section_code: section.section_code, 
                    instructors: section.instructors, 
                    meetings: section.meetings, 
                    open_seats: section.open_seats, 
                    total_seats: section.total_seats, 
                    waitlist: section.waitlist, 
                    holdfile: section.holdfile 
                },
                hover: false,
                differences: SelectionDifferences::all_false(),
                color_number: i as i32 //changes the color for each section
            })
        }).collect())
    ).collect();
}
