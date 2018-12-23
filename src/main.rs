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
extern crate cpuprofiler;
extern crate regex;
extern crate unicode_segmentation;
use cpuprofiler::PROFILER;
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("AoC 2018")
        .arg(
            Arg::with_name("prof_file")
                .short("p")
                .long("prof-file")
                .value_name("FILE")
                .help("Enable profiling and write to specified file")
                .takes_value(true),
        )
        .get_matches();
    let prof_file_value = matches.value_of("prof_file");
    if let Some(prof_file) = prof_file_value {
        PROFILER
            .lock()
            .unwrap()
            .start(prof_file)
            .expect("Could not start profiling");
    }
    assert_eq!(day1::run(), (474, 137_041));
    assert_eq!(day2::run(), (4693, "pebjqsalrdnckzfihvtxysomg".to_string()));
    assert_eq!(day3::run(), 97218);
    assert_eq!(day4::run(), 21956);
    assert_eq!(day5::run(), 9116);
    assert_eq!(day6::run(), 3882);
    assert_eq!(day7::run(), "LFMNJRTQVZCHIABKPXYEUGWDSO");
    assert_eq!(day8::run(), 37439);
    if prof_file_value.is_some() {
        PROFILER
            .lock()
            .unwrap()
            .stop()
            .expect("Could not stop profiling");
    }
}
