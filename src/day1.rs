use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod problem1{
    fn read_input() {
	    let f = File::open("input/input1.txt")?;
	    let reader = BufReader::new(f);

	    for line in reader.lines() {
	        println!("{}", line?);
	    }
    }
}