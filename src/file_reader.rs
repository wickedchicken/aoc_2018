use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_input(filename: String) -> Box<Iterator<Item = String>> {
    let f = File::open(filename).expect("Could not open file!");
    let reader = BufReader::new(f);

    Box::new(reader.lines().map(|l| l.unwrap()))
}
