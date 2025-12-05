/**
 * This file is part of Jupiterp. For terms of use, please see the file
 * called LICENSE at the top level of the Jupiterp source tree (online at
 * https://github.com/atcupps/Jupiterp/LICENSE).
 * Copyright (C) 2024 Andrew Cupps
 */

use crate::{serde_structs::Course, sorting_structs::*};

pub fn get_potential_schedules(desired_courses: Vec<Course>) -> Vec<Vec<Section>> {
    let mut desired_sections: Vec<Vec<Section>> = desired_courses
        .into_iter()
        .map(|c| c.to_sectionbits())
        .collect();

    //sort courses by number of sections to speed up backtracking
    desired_sections.sort_by_key(|sections| sections.len());

    //priorize high quality sections by sorting by (openseats - waitlist - holdfile)
    for course in desired_sections.iter_mut() {
        course.sort_by_key(|section| {
            -(section.open_seats as i32)
                + (section.waitlist as i32)
                + (section.holdfile.unwrap_or(0) as i32)
        });
    }

    let mut results: Vec<Vec<Section>> = Vec::new();
    let mut current: Vec<Section> = Vec::new();
    let mut checks: u64 = 0;

    backtrack(
        0,
        &mut current,
        &desired_sections,
        &mut results,
        &mut checks,
    );

    //sort results by seat quality like before
    results.sort_by_key(|schedule| {
        schedule
            .iter()
            .map(|section| {
                -(section.open_seats as i32)
                    + (section.waitlist as i32)
                    + (section.holdfile.unwrap_or(0) as i32)
            })
            .sum::<i32>()
    });

    return results;
}

///backtracking function for get_potential_schedules
fn backtrack(
    course_idx: usize,
    current_schedule: &mut Vec<Section>,
    courses: &Vec<Vec<Section>>,
    results: &mut Vec<Vec<Section>>,
    checks: &mut u64,
) {
    if course_idx == courses.len() {
        results.push(current_schedule.clone());
        return;
    }

    if *checks >= 5000 {
        return;
    } //keep computation time down

    for new_section in &courses[course_idx] {
        let mut has_conflict: bool = false;
        for existing_section in current_schedule.iter() {
            *checks += 1;

            if is_conflict(new_section, existing_section) {
                has_conflict = true;
                break;
            }
        }

        if !has_conflict {
            current_schedule.push(new_section.clone());
            backtrack(course_idx + 1, current_schedule, courses, results, checks);
            current_schedule.pop();
        }
    }
}

///Checks if the meeting times of two given sections overlap
pub fn is_conflict(section1: &Section, section2: &Section) -> bool {
    section1
        .classtimes
        .iter()
        .zip(section2.classtimes.iter())
        .any(|(x, y)| (x & y) != 0)
}
