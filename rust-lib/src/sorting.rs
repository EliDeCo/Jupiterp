use crate::{serde_structs::Course, sorting_structs::*};
use std::collections::HashMap;


//earlist and latest time to go to class
const EARLIEST: u32 = 800;
const LATEST: u32 = 2200;

pub fn get_potential_schedules(
    desired_courses: Vec<Course>,
) -> Vec<Vec<Section>> {

    let mut desired_sections: Vec<Vec<Section>> = desired_courses.into_iter().map(|c|c.to_sectionbits()).collect();
    /* 
    //remove sections that break earliest/latest filter,
    for course in desired_sections.iter_mut() {
        course.retain(|section|{
            
            let start_slot = get_slot_2(EARLIEST, 8, 15);
            let end_slot = get_slot_2(LATEST, 8, 15) + 1;

            let allowed_len = end_slot.saturating_sub(start_slot);
            let allowed_mask: u64 = (((1u128 << allowed_len) - 1) as u64) << start_slot;

            for day in section.classtimes {
                if (day & !allowed_mask) != 0 {
                    return false;
                }
            }
            return true;
        });
    }
    */

    desired_sections.sort_by_key(|sections|sections.len());
    

    let mut conflict_memo: HashMap<String,bool> = HashMap::new();
    let mut results: Vec<Vec<Section>> = Vec::new();
    let mut current: Vec<Section> = Vec::new();


    backtrack(0, &mut current, &desired_sections, &mut results, &mut conflict_memo);

    return results;
}

fn get_slot_2(time: u32, earliest: u32, interval: u32) -> u32 {
    let hours: u32 = time / 100;
    let minutes: u32 = time % 100;
    return (hours * 60 + minutes - (earliest * 60)) / interval;
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
    section1.classtimes
        .iter()
        .zip(section2.classtimes.iter())
        .any(|(x,y)|(x & y) != 0)
}

///turns a pair of sections into a unique id with consistent order
pub fn pair_to_id(section1: &Section, section2: &Section) -> String {
    let mut both: [&Section; 2] = [section1,section2];
    both.sort_by_key(|s|&s.course);
    format!("{}{}{}{}", both[0].course, both[0].section, both[1].course, both[1].section)
}



