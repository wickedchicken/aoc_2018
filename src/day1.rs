use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_input() {
    let f = File::open("input/input1.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line?);
    }
}
