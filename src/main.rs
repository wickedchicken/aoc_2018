mod day1;
mod day2;
mod day3;
mod file_reader;
extern crate regex;

fn main() {
    assert_eq!(day1::run(), 474);
    assert_eq!(day2::run(), 4693);
    assert_eq!(day3::run(), 97218);
}
