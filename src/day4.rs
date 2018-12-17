use crate::file_reader::read_input;
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DateTime {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

impl DateTime {
    fn parse(line: &str) -> DateTime {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+)-(\d+) (\d+):(\d+)").unwrap();
        }
        let captures = RE.captures(&line).unwrap();
        DateTime {
            year: captures[1].parse().unwrap(),
            month: captures[2].parse().unwrap(),
            day: captures[3].parse().unwrap(),
            hour: captures[4].parse().unwrap(),
            minute: captures[5].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum GuardLine {
    Begin(DateTime, i32),
    FallsAsleep(DateTime),
    WakesUp(DateTime),
}

impl GuardLine {
    fn parse(line: &str) -> GuardLine {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(.+)\] (.+)").unwrap();
        }
        let captures = RE.captures(line.trim()).unwrap();

        match &captures[2] {
            "falls asleep" => GuardLine::FallsAsleep(DateTime::parse(&captures[1])),
            "wakes up" => GuardLine::WakesUp(DateTime::parse(&captures[1])),
            _ => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
                }
                let guard_id = RE.captures(&captures[2]).unwrap()[1].parse().unwrap();
                GuardLine::Begin(DateTime::parse(&captures[1]), guard_id)
            }
        }
    }
}

fn build_time_map(actions: &[GuardLine]) -> HashMap<i32, HashMap<i32, i32>> {
    let mut time_map = HashMap::new();
    let mut current_guard: Option<i32> = None;
    let mut last_asleep_time: Option<i32> = None;
    for action in actions {
        match action {
            GuardLine::Begin(_date_time, id) => {
                current_guard = Some(*id);
            }
            GuardLine::FallsAsleep(date_time) => {
                last_asleep_time = Some(date_time.minute);
            }
            GuardLine::WakesUp(date_time) => {
                for minute in last_asleep_time.unwrap()..(date_time.minute) {
                    let guard_map = time_map
                        .entry(current_guard.unwrap())
                        .or_insert_with(HashMap::new);
                    let minute_entry = guard_map.entry(minute).or_insert(0);
                    *minute_entry += 1;
                }
                last_asleep_time = None;
            }
        }
    }
    time_map
}

pub fn run() -> i32 {
    let mut guard_actions = Vec::new();

    for line in read_input("input/input4.txt".to_string()) {
        guard_actions.push(GuardLine::parse(&line));
    }

    // couldn't get sort_unstable_by_key to work due to some kind of lifetime thing
    guard_actions.sort_unstable_by(|a, b| {
        let a_key = match a {
            GuardLine::Begin(date_time, _id) => date_time,
            GuardLine::FallsAsleep(date_time) => date_time,
            GuardLine::WakesUp(date_time) => date_time,
        };
        let b_key = match b {
            GuardLine::Begin(date_time, _id) => date_time,
            GuardLine::FallsAsleep(date_time) => date_time,
            GuardLine::WakesUp(date_time) => date_time,
        };
        a_key.cmp(b_key)
    });

    let map = build_time_map(&guard_actions);

    let sleepiest_guard: (&i32, i32) = map
        .iter()
        .map(|(k, v)| (k, v.values().sum()))
        .max_by_key(|x| x.1)
        .unwrap();

    let sleepiest_minute = map[sleepiest_guard.0].iter().max_by_key(|x| x.1).unwrap();

    sleepiest_guard.0 * sleepiest_minute.0
}
