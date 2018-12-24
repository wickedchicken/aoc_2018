mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod file_reader;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate unicode_segmentation;

fn main() {
    assert_eq!(day1::run(), (474, 137_041));
    assert_eq!(day2::run(), (4693, "pebjqsalrdnckzfihvtxysomg".to_string()));
    assert_eq!(day3::run(), (97218, 717));
    assert_eq!(day4::run(), 21956);
    assert_eq!(day5::run(), 9116);
    assert_eq!(day6::run(), 3882);
    assert_eq!(day7::run(), "LFMNJRTQVZCHIABKPXYEUGWDSO");
    assert_eq!(day8::run(), 37439);
    assert_eq!(day9::run(419, 71052), 412_117);
    let (day10_res, day10_secs) = day10::run();
    assert!((day10_res - 549.0).abs() < std::f64::EPSILON);
    assert_eq!(day10_secs, 10274);
}
