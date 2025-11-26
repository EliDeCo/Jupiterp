use crate::{log_str, serde_structs::{ClassMeetingFull, Course, Coursedata}, sorting_structs::*};
use haversine_rs::{distance, point::Point, units::Unit};
use std::{collections::{HashMap, HashSet}, fmt::format, result, u32};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

//walk speed in meters per second
const WALK_SPEED: f32 = 1.42;
//const WALK_SPEED: f32 = 100.;
//earlist and latest time to go to class
const EARLIST: u32 = 100;
const LATEST: u32 = 2300;
//constant to convert from degrees to meters
const LAT_TO_M: f32 = 111_111.;
const LON_TO_M: f32 = 86_610.;


/// Convert HHMM -> total minutes since midnight and compare
pub fn time_between(first: u32, second: u32) -> u32 {
    let to_minutes = |time: u32| -> u32 {
        let hours = time / 100;
        let minutes = time % 100;
        hours * 60 + minutes
    };

    return to_minutes(second) - to_minutes(first);
}

/// takes two sections and determines if they have overlapping time slots, unwalkable, or too early or late (outdated)
pub fn is_conflict_legacy(
    section1: &Section,
    section2: &Section,
    buildings: &HashMap<String, BuildingData>,
    walk_speed: f32,
    earliest: u32,
    latest: u32,
) -> bool {

    for day in 1..6 {
        //if the day is present in both sections
        if let (Some(times_from_1), Some(times_from_2)) =
            (section1.classtimes.get(&day), section2.classtimes.get(&day))
        {
            //compare every meeting on every day from both courses with each other
            'loop1: for times1 in times_from_1 {
                'loop2: for times2 in times_from_2 {
                    //if the class starts or ends to early, this section is deemed a conflict
                    if times1.start < earliest
                        || times2.start < earliest
                        || times1.end > latest
                        || times2.end > latest
                    {
                        return true;
                    }

                    //if any start or end times are shared, they overlap
                    if times1.start == times2.start
                        || times1.start == times2.end
                        || times1.end == times2.start
                        || times1.end == times2.end
                    {
                        return true;
                    }

                    //order them by which one starts first
                    let mut chronological: [&StartEnd; 2] = [times1, times2];
                    chronological.sort_by(|a, b| a.start.cmp(&b.start));

                    let first: &StartEnd = chronological[0];
                    let second: &StartEnd = chronological[1];

                    //if the first one ends after the second one starts, they overlap
                    if first.end > second.start {
                        return true;
                    } else {
                        //skip if one of the courses is asyncronous or unknown location
                        if first.building == "OnlineSync" || first.building == "TBA" {
                            continue 'loop1;
                        } 
                        if second.building == "OnlineSync" || second.building == "TBA" {
                            continue 'loop2;
                        }

                        //if both classes are in person, test to see if there is enough time to walk

                        let time_between: u32 = time_between(first.end, second.start) * 60; //time between classes in seconds


                        let pos1: Point = Point::new(
                            buildings.get(&first.building).unwrap_or_else(|| {
                                log_str(&format!("Building: \"{}\" not in database", first.building));
                                &BuildingData { long: 0.0, lat: 0.0 }
                            }).lat as f64,
                            buildings.get(&first.building).unwrap_or(
                                &BuildingData { long: 0.0, lat: 0.0 }
                            ).long as f64,
                        );

                        let pos2: Point = Point::new(
                            buildings.get(&first.building).unwrap_or(
                                &BuildingData { long: 0.0, lat: 0.0 }
                            ).lat as f64,
                            buildings.get(&second.building).unwrap_or(
                                &BuildingData { long: 0.0, lat: 0.0 }
                            ).long as f64,
                        );

                        let pos3: Point = Point::new(
                            buildings.get(&second.building).unwrap_or_else(|| {
                               log_str(&format!("Building: \"{}\" not in database", second.building));
                                &BuildingData { long: 0.0, lat: 0.0 }
                            }).lat as f64,
                            buildings.get(&second.building).unwrap_or(
                                &BuildingData { long: 0.0, lat: 0.0 }
                            ).long as f64,
                        );

                        //computes the maximum distance: we cannot take a straghit line, and must go straight East or West, then straight north or south
                        //this simulates real walking where we often have to follow horizontal and vertical roads and paths
                        let max_distance: f32 = distance(pos1, pos2, Unit::Meters) as f32
                            + distance(pos2, pos3, Unit::Meters) as f32;
                        //println!("Distance between {} and {} = {}", first.building, second.building, max_distance);
                        /*
                        if tester && time_between == 600 {
                            println!("-----------------------------------");
                            println!("Distance between {} and {} = {}", first.building, second.building, max_distance);
                            let walk_time = max_distance / walk_speed;
                            println!("Walk time in minutes: {}", walk_time/60.);

                            //println!("How early in minutes: {}", (time_between as f32 /60.) -(walk_time/60.))
                            println!("{}-{}, {}-{}", first.start, first.end, second.start, second.end);
                            println!("Time between: {}", time_between/60)
                        }
                        */

                        if time_between as f32 - (max_distance / walk_speed) < 300. {
                            //if we can't get there 5 minutes early, deem this section as a conflict
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

///Generates all potential schedules from the desired courses (old)
pub fn get_potential_schedules_legacy(
    desired_courses: CourseMap,
    buildings: &BuildingMap,
) -> Vec<Schedule> {
    //return nothing if empty
    if desired_courses.is_empty() {
        return Vec::new();
    }

    // Convert to a Vec so we can index only the first course.
    let mut desired_courses: Vec<_> = desired_courses.into_iter().collect();
    desired_courses.sort_by(|a, b| a.0.cmp(&b.0));
    //sorting makes the process deterministic for testing


    let mut potential_schedules: Vec<Schedule> = Vec::new();
    //initialize will all sections of the first course
    for section in desired_courses[0].1.values() {
        potential_schedules.push(vec![section.clone()]);
    }

    for (_, sections) in desired_courses.iter().skip(1) {
        let mut new_potential_schedules: Vec<Schedule> = Vec::new();
        for (_, new_section) in sections {
            'schedule_loop: for schedule in &potential_schedules {
                for section in schedule {
                    if is_conflict_legacy(
                        section,
                        new_section,
                        &buildings,
                        WALK_SPEED,
                        EARLIST,
                        LATEST,
                    ) {
                        continue 'schedule_loop;
                    }
                }
                //if we reach here, every section in the currently selected schedule is compatible with the new section
                //this means that this schedule is valid, as it can hold 1 of every course we have iterated through at this point in time
                let mut new_schedule: Vec<Section> = schedule.clone();
                new_schedule.push(new_section.clone());
                //sort courses alphabetically within their schedules
                new_schedule.sort_by(|a, b| a.course.cmp(&b.course));
                new_potential_schedules.push(new_schedule);
            }
        }
        potential_schedules = new_potential_schedules;
    }

    potential_schedules
}

//disregard
//keeping this here in case this funcionality is implimented in the future
#[allow(dead_code)] 
///Computes possible alternates for all the given potential schedules
pub fn schedules_with_alternatives(
    potential_schedules: Vec<Schedule>,
    buildings: &BuildingMap,
    alternates: &CourseMap,
) -> Vec<ScheduleWithAlternates> {
    let mut schedules_with_alternates: Vec<ScheduleWithAlternates> = Vec::new();
    for schedule in potential_schedules {
        //generate the schedule with all possible alternates
        let mut single_with_alts: Vec<(Section, Vec<Section>)> = schedule
            .iter()
            .map(|s| {(
                s.clone(),
                s.find_alt(
                    schedule.clone(),
                    buildings,
                    WALK_SPEED,
                    EARLIST,
                    LATEST,
                    alternates,
                ),
            )})
            .collect();

        //sort alphabetically by course
        single_with_alts.sort_by(|a, b| a.0.course.cmp(&b.0.course));

        schedules_with_alternates.push(single_with_alts);
    }
    schedules_with_alternates
}

#[allow(dead_code)] 
///compute median of a collection of floats
fn median(numbers: &Vec<f32>) -> f32 {
    let mut numbers = numbers.clone();
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = numbers.len();

    if len == 0 {
        return f32::NAN; // or panic!("empty list") if that fits your case
    }

    if len % 2 == 1 {
        numbers[len / 2]
    } else {
        let mid = len / 2;
        (numbers[mid - 1] + numbers[mid]) / 2.0
    }
}

//TODO: FIX EVERYTHING AND USE REFERENCES FOR THE LOVE OF GOD

pub fn get_potential_schedules(
    desired_courses: Vec<Course>,
    buildings: JsValue,
) -> Vec<Schedule> {

    let mut desired_sections: Vec<Vec<Section>> = desired_courses.into_iter().map(|c|c.to_sections()).collect();

    //remove sections that break earlist/latest filter,
    for course in desired_sections.iter_mut() {
        course.retain(|section|{
            let mut earliest: u32 = u32::MAX;
            let mut latest: u32 = 0;

            for start_end in section.classtimes.values().flatten() {
                if start_end.start < earliest { earliest = start_end.start }
                if start_end.end > latest { latest = start_end.end }
            }

            if earliest == u32::MAX {
                return false;
            }

            return earliest  >= EARLIST && latest <= LATEST;
        });
    }

    desired_sections.sort_by_key(|sections|sections.len());
    

    let mut conflict_memo: HashMap<String,bool> = HashMap::new();
    let mut results: Vec<Vec<Section>> = Vec::new();
    let mut current: Vec<Section> = Vec::new();


    backtrack(0, &mut current, &desired_sections, &mut results, &mut conflict_memo);

    return results;
}

///backtracking function for get_potential_schedules
fn backtrack(
    course_idx: usize,
    current_schedule: &mut Vec<Section>,
    courses: &Vec<Vec<Section>>,
    results: &mut Vec<Vec<Section>>,
    conflict_memo: &mut HashMap<String,bool>,
) {

    if course_idx == courses.len() {
        results.push(current_schedule.clone());
        return;
    }

    for new_section in &courses[course_idx] {
        let mut has_conflict: bool = false;
        for existing_section in current_schedule.iter() {
            let key = pair_to_id(new_section, existing_section);
            let conflict = match conflict_memo.get(&key) {
                Some(&b) => b,
                None => {
                    let b = is_conflict(new_section, existing_section);
                    conflict_memo.insert(key, b);
                    b
                }
            };

            if conflict {
                has_conflict = true;
                break;
            }
        }

        if !has_conflict {
            current_schedule.push(new_section.clone());
            backtrack(course_idx + 1, current_schedule, courses, results, conflict_memo);
            current_schedule.pop();
        }
    }
}

///Checks if the meeting times of two given sections overlap
pub fn is_conflict(
    section1: &Section,
    section2: &Section,
) -> bool {

    return true;
}

///turns a pair of sections into a unique id with consistent order
pub fn pair_to_id(section1: &Section, section2: &Section) -> String {
    let mut both: [&Section; 2] = [section1,section2];
    both.sort_by_key(|s|&s.course);
    format!("{}{}{}{}", both[0].course, both[0].section, both[1].course, both[1].section)
}



