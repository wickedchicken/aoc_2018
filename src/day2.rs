use file_reader::read_input;
use std::collections::HashMap;

pub fn run() -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for line in read_input("input/input2.txt".to_string()) {
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

    return twos * threes;
}
