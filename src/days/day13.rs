use crate::file_reader::read_input;
use regex::Regex;

#[derive(Debug)]
struct HorizLine {
    startx: i32,
    endx: i32,
    y: i32,
}

impl HorizLine {
    fn get_horiz_lines(lines: &mut Iterator<Item = String>) -> Vec<HorizLine> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[/\\][-+]+[/\\]").unwrap();
        }
        let mut result = Vec::new();
        for (i, line) in lines.enumerate() {
            result.extend(
                RE.find_iter(&line).map(|x| HorizLine{startx: x.start() as i32, endx: x.end() as i32, y: i as i32}).collect::<Vec<HorizLine>>())
        }
        result
    }
}

pub fn run() -> i32 {
    let horiz_lines = HorizLine::get_horiz_lines(&mut read_input("input/input13.txt".to_string()));
    println!("{:?}", horiz_lines);
    0
}
