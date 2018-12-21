mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod file_reader;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate unicode_segmentation;

fn main() {
    assert_eq!(day1::run(), (474, 137_041));
    assert_eq!(day2::run(), (4693, "pebjqsalrdnckzfihvtxysomg".to_string()));
    assert_eq!(day3::run(), 97218);
    assert_eq!(day4::run(), 21956);
    assert_eq!(day5::run(), 9116);
    assert_eq!(day6::run(), 3882);
    assert_eq!(day7::run(), "LFMNJRTQVZCHIABKPXYEUGWDSO");
    assert_eq!(day8::run(), 37439);
}
