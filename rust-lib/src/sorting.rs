/**
 * This file is part of Jupiterp. For terms of use, please see the file
 * called LICENSE at the top level of the Jupiterp source tree (online at
 * https://github.com/atcupps/Jupiterp/LICENSE).
 * Copyright (C) 2024 Andrew Cupps
 */

use crate::{serde_structs::Course, sorting_structs::*};

///Converts input Courses into per-course Vec<Section> lists, sorted for
///backtracking: courses ordered ascending by section count (fail fast on
///the most constrained course first), and each course's sections ordered
///by seat-quality (best options tried first).
pub fn prepare_sections(desired_courses: Vec<Course>) -> Vec<Vec<Section>> {
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

    desired_sections
}

///Runs the backtracking search over `courses` (as produced by
///`prepare_sections`) and returns every discovered conflict-free schedule
///as borrowed Sections, so no Section is ever copied.
pub fn get_potential_schedules<'a>(courses: &'a Vec<Vec<Section>>) -> Vec<Vec<&'a Section>> {
    let mut results: Vec<Vec<&'a Section>> = Vec::new();
    let mut current: Vec<&'a Section> = Vec::new();
    let mut checks: u64 = 0;

    backtrack(0, &mut current, courses, &mut results, &mut checks);

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
fn backtrack<'a>(
    course_idx: usize,
    current_schedule: &mut Vec<&'a Section>,
    courses: &'a Vec<Vec<Section>>,
    results: &mut Vec<Vec<&'a Section>>,
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
        for &existing_section in current_schedule.iter() {
            *checks += 1;

            if is_conflict(new_section, existing_section) {
                has_conflict = true;
                break;
            }
        }

        if !has_conflict {
            current_schedule.push(new_section);
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
