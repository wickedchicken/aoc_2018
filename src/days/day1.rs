use crate::file_reader::read_input;
use std::collections::HashSet;

pub fn run() -> (i32, i32) {
    let mut accumulator = 0;
    let mut twice: Option<i32> = None;
    let mut seen = HashSet::new();
    seen.insert(0);
    for line in read_input("input/input1.txt".to_string()) {
        let line_number: i32 = line.trim().parse().unwrap();
        accumulator += line_number;
        if seen.contains(&accumulator) && twice.is_none() {
            twice = Some(accumulator);
        }
        seen.insert(accumulator);
    }

    // keep looping to find the repeating value
    let mut new_accumulator = accumulator;
    while let None = twice {
        for line in read_input("input/input1.txt".to_string()) {
            let line_number: i32 = line.trim().parse().unwrap();
            new_accumulator += line_number;
            if seen.contains(&new_accumulator) && twice.is_none() {
                twice = Some(new_accumulator);
                break;
            }
            seen.insert(new_accumulator);
        }
    }
    (accumulator, twice.unwrap())
}
