mod day1;
mod day2;
mod day3;
mod day4;
mod file_reader;
#[macro_use]
extern crate lazy_static;
extern crate regex;

fn main() {
    assert_eq!(day1::run(), 474);
    assert_eq!(day2::run(), 4693);
    assert_eq!(day3::run(), 97218);
    assert_eq!(day4::run(), 21956);
}
