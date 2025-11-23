use serde::Deserialize;
use crate::sorting_structs::*;
use std::collections::HashMap;


#[derive(Deserialize)]
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


impl Course {
    pub fn to_coursemap(self) -> (String, SectionMap) {
        let mut section_map: SectionMap = HashMap::new();
        if let Some(sections) = self.sections {
            for sec in sections {
                let professor: ProfData = ProfData { name: sec.instructors.join(" & "), rating: 0.0 };
                let classtimes: Classtimes = classmeet_convert(sec.meetings);
                let course: String = sec.course_code;
                let section: String = sec.section_code.clone();
                let seats: [u32; 3] = [sec.total_seats, sec.open_seats, sec.waitlist];

                section_map.insert(sec.section_code, Section { professor, classtimes, course, section, seats });
            }
            return (self.course_code, section_map);
        } else {
            return (self.course_code, HashMap::new());
        }
    }
}


#[derive(Deserialize)]
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
    //holdfile: Option<u32>
}


#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct ClassMeeting {
    classtime: Classtime,
    location: Location,
}


#[derive(Deserialize, Default)]
pub struct Classtime {
    pub days: String,
    pub start: f32,
    pub end: f32,
}

#[derive(Deserialize, Default)]
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
            _ => continue,
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
            start: fmt_startend_time(meeting.classtime.start), 
            end: fmt_startend_time(meeting.classtime.end)
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

fn fmt_startend_time(time: f32) -> u32 {
    let hours = time.floor() as u32;
    let minutes = ((time - hours as f32) * 60.0).round() as u32;
    hours * 100 + minutes
}