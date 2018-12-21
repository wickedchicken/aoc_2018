use crate::file_reader::read_input;
use std::collections::HashMap;

fn compare_str(a: &str, b: &str) -> String {
    let a_chars = a.chars();
    let mut b_chars = b.chars();

    assert_eq!(a.len(), b.len());

    let mut matches = "".to_string();
    for a_char in a_chars {
        let b_char = b_chars.next().unwrap();
        if a_char == b_char {
            matches.push(a_char);
        }
    }

    matches
}

pub fn run() -> (i32, String) {
    let mut twos = 0;
    let mut threes = 0;

    let mut box_ids: Vec<String> = Vec::new();
    let mut output: Option<String> = None;

    for line in read_input("input/input2.txt".to_string()) {
        if output.is_none() {
            let trimmed = line.trim().to_string();
            for id in box_ids.iter() {
                let common = compare_str(&trimmed, id);
                if common.len() == &trimmed.len() - 1 {
                    output = Some(common.clone());
                    break;
                }
            }
            box_ids.push(trimmed);
        }

        let mut seen_characters = HashMap::new();
        for c in line.chars() {
            let stat = seen_characters.entry(c).or_insert(0);
            *stat += 1;
        }
        let mut seen_two = false;
        let mut seen_three = false;
        for val in seen_characters.values() {
            if *val == 3 && !seen_three {
                threes += 1;
                seen_three = true;
            } else if *val == 2 && !seen_two {
                twos += 1;
                seen_two = true;
            }
        }
    }

    (twos * threes, output.unwrap())
}
