mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod file_reader;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate unicode_segmentation;

fn main() {
    assert_eq!(day1::run(), 474);
    assert_eq!(day2::run(), 4693);
    assert_eq!(day3::run(), 97218);
    assert_eq!(day4::run(), 21956);
    assert_eq!(day5::run(), 9116);
}
