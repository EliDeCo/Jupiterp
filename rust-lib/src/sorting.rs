use crate::sorting_structs::*;
use haversine_rs::{distance, point::Point, units::Unit};
use std::collections::HashMap;

//walk speed in meters per second
const WALK_SPEED: f32 = 1.42;
//const WALK_SPEED: f32 = 100.;
//earlist and latest time to go to class
const EARLIST: u32 = 1000;
const LATEST: u32 = 1500;


/// Convert HHMM -> total minutes since midnight and compare
pub fn time_between(first: u32, second: u32) -> u32 {
    let to_minutes = |time: u32| -> u32 {
        let hours = time / 100;
        let minutes = time % 100;
        hours * 60 + minutes
    };

    return to_minutes(second) - to_minutes(first);
}

/// takes two sections and determines if they have overlapping time slots, unwalkable, or too early or late
pub fn is_conflict(
    section1: &Section,
    section2: &Section,
    buildings: &HashMap<String, BuildingData>,
    walk_speed: f32,
    earliest: u32,
    latest: u32,
) -> bool {
    //TO DEBUG:
    let tester: bool;
    if (0 == 1)
        && section1.course == "FREN103"
        && section1.section == "0301"
        && section2.course == "PHYS260"
        && section2.section == "0201"
    {
        tester = true;
    } else {
        tester = false;
    }

    for day in 1..6 {
        //if the day is present in both sections
        if let (Some(times_from_1), Some(times_from_2)) =
            (section1.classtimes.get(&day), section2.classtimes.get(&day))
        {
            //compare every meeting on every day from both courses with each other
            for times1 in times_from_1 {
                for times2 in times_from_2 {
                    //if the class starts or ends to early, this section is deemed a conflict
                    if times1.start < earliest
                        || times2.start < earliest
                        || times1.end > latest
                        || times2.end > latest
                    {
                        if tester {
                            println!(
                                "On day {}, {}-{} OR {}-{} ends too late/starts too early",
                                day,
                                section1.course,
                                section1.section,
                                section2.course,
                                section2.section
                            );
                        }
                        return true;
                    }

                    //if any start or end times are shared, they overlap
                    if times1.start == times2.start
                        || times1.start == times2.end
                        || times1.end == times2.start
                        || times1.end == times2.end
                    {
                        if tester {
                            println!(
                                "On day {}, {}-{} conflicts with {}-{}: Start/End Shared",
                                day,
                                section1.course,
                                section1.section,
                                section2.course,
                                section2.section
                            );
                        }
                        return true;
                    }

                    //order them by which one starts first
                    let mut chronological: [&StartEnd; 2] = [times1, times2];
                    chronological.sort_by(|a, b| a.start.cmp(&b.start));

                    let first: &StartEnd = chronological[0];
                    let second: &StartEnd = chronological[1];

                    //if the first one ends after the second one starts, they overlap
                    if first.end > second.start {
                        if tester {
                            println!(
                                "On day {}, {}-{} conflicts with {}-{}: Overlap Trouble",
                                day,
                                section1.course,
                                section1.section,
                                section2.course,
                                section2.section
                            );
                        }
                        return true;
                    } else {
                        //test to see if there is enough time to walk
                        let time_between: u32 = time_between(first.end, second.start) * 60; //time between classes in seconds

                        //println!("{}", &first.building);
                        let pos1: Point = Point::new(
                            buildings[&first.building].lat as f64,
                            buildings[&first.building].long as f64,
                        );
                        //println!("{}", &second.building);
                        let pos2: Point = Point::new(
                            buildings[&second.building].lat as f64,
                            buildings[&first.building].long as f64,
                        );
                        let pos3: Point = Point::new(
                            buildings[&second.building].lat as f64,
                            buildings[&second.building].long as f64,
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
                            if tester {
                                println!(
                                    "On day {}, {}-{} conflicts with {}-{}: Cant get there in time",
                                    day,
                                    section1.course,
                                    section1.section,
                                    section2.course,
                                    section2.section
                                );
                            }
                            //println!("Cant walk");
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

#[allow(dead_code)] //Rating may be done here in the future
///gives a rating of the inputted schedule for ordering
pub fn rating(schedule: &ScheduleWithAlternates, all_alternates: &Vec<String>) -> f32 {
    //Sum of all professor ratings
    let prof_rating: f32 = schedule.iter().map(|(s, _)| s.professor.rating).sum();

    //list of the average ratings for each alternate
    let alt_ratings: Vec<f32> = schedule
        .iter()
        .map(|(_, a)| {
            if a.is_empty() {
                0.0
            } else {
                let sum: f32 = a.iter().map(|s| s.professor.rating).sum();
                sum / a.len() as f32
            }
        })
        .collect();

    //average alternate rating
    let av_alt_rating: f32 = alt_ratings.iter().sum::<f32>() / alt_ratings.len() as f32;

    //rewards a schedule for giving freedom in which alternate courses are availible and when they can be taken
    let mut alternate_diversity_rating: f32 = 0.;
    for (_, alts) in schedule {
        //for each course in the schedule
        let course_alts: Vec<String> = alts.iter().map(|s| s.course.clone()).collect();
        let mut counts: HashMap<String, usize> = HashMap::new(); //Amount of times each alternate course shows up
        for alt in course_alts {
            *counts.entry(alt).or_insert(0) += 1;
        }
        //get reward based on how many alternate courses (not sections) are availible for this course
        alternate_diversity_rating += counts.len() as f32;
        //get a reward based on the median number of sections per alternate course (rewards diverse options without overvaluing outliers)
        for given_alternate in all_alternates.clone() {
            //insert  zeroes for sections not included
            counts.entry(given_alternate).or_insert(0);
        }
        let section_nums: Vec<f32> = counts.values().copied().map(|v| v as f32).collect();
        alternate_diversity_rating += median(&section_nums);
    }
    //temporarily display all three ratings for debugging
    /* 
    println!(
        "Prof Rating: {}, Av Alt Rating: {}, Alt Diversity Rating: {}",
        prof_rating, av_alt_rating, alternate_diversity_rating
    );
    */
    let _ =  prof_rating + av_alt_rating + alternate_diversity_rating;
    return prof_rating;
}

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

///Generates all potential schedules from the desired courses
pub fn get_potential_schedules(
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
            'schedule_loop: for schedule in potential_schedules.clone() {
                for section in schedule.clone() {
                    if is_conflict(
                        &section,
                        &new_section,
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

#[allow(dead_code)] //functionality for generating alternates may be added in the future
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
            .map(|s| {
                (
                    s.clone(),
                    s.find_alt(
                        schedule.clone(),
                        buildings,
                        WALK_SPEED,
                        EARLIST,
                        LATEST,
                        alternates,
                    ),
                )
            })
            .collect();

        //sort alphabetically by course
        single_with_alts.sort_by(|a, b| a.0.course.cmp(&b.0.course));

        schedules_with_alternates.push(single_with_alts);
    }

    //sort by rating, highest to lowest
    let alternates: Vec<String> = alternates.keys().cloned().collect();
    schedules_with_alternates.sort_by(|a, b| {
        rating(b, &alternates)
            .partial_cmp(&rating(a, &alternates))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    schedules_with_alternates
}
