use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_input() -> Box<Iterator<Item=String>> {
    let f = File::open("input/input1.txt").expect("Could not open file!");
    let reader = BufReader::new(f);

    Box::new(reader.lines().map(|l| l.unwrap()))
}
