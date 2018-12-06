#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::{Regex, RegexSet};
use std::collections::HashMap;
use std::fs;

enum Action {
    BeganShift,
    FellAsleep,
    WokeUp,
}

fn parse_guard(line: &str) -> (i32, Action, Option<i32>) {
    lazy_static! {
        static ref time_re: Regex =
            Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})]").unwrap();
    }

    lazy_static! {
        static ref guard_re: Regex = Regex::new(&format!(
            r"^{} Guard #(\d+) begins shift$",
            time_re.as_str()
        ))
        .unwrap();
    }

    lazy_static! {
        static ref re_set: RegexSet = RegexSet::new(&[
            &format!(r"^{} Guard #(\d+) begins shift$", time_re.as_str()),
            &format!(r"^{} falls asleep$", time_re.as_str()),
            &format!(r"^{} wakes up$", time_re.as_str())
        ])
        .unwrap();
    }
    let matches: Vec<_> = re_set.matches(line).into_iter().collect();
    let cap = time_re.captures(line).unwrap();
    let minute = cap[5].parse::<i32>().unwrap();

    let guard = if matches[0] == 0 {
        Some(guard_re.captures(line).unwrap()[6].parse::<i32>().unwrap())
    } else {
        None
    };

    let action = match matches[0] {
        0 => Action::BeganShift,
        1 => Action::FellAsleep,
        _ => Action::WokeUp,
    };

    (minute, action, guard)
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines: Vec<&str> = contents.lines().collect();
    lines.sort_unstable();

    let mut guards: HashMap<i32, (i32, HashMap<i32, i32>)> = HashMap::new();
    let mut current_guard = 0;
    let mut fell_asleep = None;

    let mut max_sleep_id = 0;
    let mut max_sleep_minutes = 0;

    for line in lines {
        let (current_min, action, guard) = parse_guard(line);

        match action {
            Action::BeganShift => {
                current_guard = guard.unwrap();
                fell_asleep = None;
            }
            Action::FellAsleep => {
                fell_asleep = Some(current_min);
            }
            Action::WokeUp => {
                if let Some(min_asleep) = fell_asleep {
                    let (total_minutes, guard_minutes) =
                        guards.entry(current_guard).or_insert((0, HashMap::new()));
                    let minutes_slept = current_min - min_asleep;

                    for minute in min_asleep..(min_asleep + minutes_slept) {
                        let minute_count = guard_minutes.entry(minute).or_insert(0);
                        *minute_count += 1;
                        *total_minutes += 1;
                    }

                    if *total_minutes > max_sleep_minutes {
                        max_sleep_id = current_guard;
                        max_sleep_minutes = *total_minutes;
                    }

                    fell_asleep = None;
                }
            }
        };
    }

    let (_, minutes) = &guards[&max_sleep_id];
    let (minute_most_slept, _) = get_minute_most_slept(minutes);
    let part1 = max_sleep_id * minute_most_slept;

    let (guard, (_, minutes)) = guards
        .into_iter()
        .max_by_key(|(_, (_, minutes))| {
            let (_, amount) = get_minute_most_slept(&minutes);
            *amount
        })
        .unwrap();

    let (minute_most_slept, _) = get_minute_most_slept(&minutes);
    let part2 = guard * minute_most_slept;

    println!("part 1: {}, part2: {}", part1, part2);
}

fn get_minute_most_slept(minutes: &HashMap<i32, i32>) -> (&i32, &i32) {
    minutes
        .into_iter()
        .max_by_key(|&(_, amount)| amount)
        .unwrap()
}
