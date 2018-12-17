use crate::file_reader::read_input;

pub fn run() -> i32 {
    let mut accumulator = 0;
    for line in read_input("input/input1.txt".to_string()) {
        let line_number: i32 = line.trim().parse().unwrap();
        accumulator += line_number;
    }
    return accumulator;
}
