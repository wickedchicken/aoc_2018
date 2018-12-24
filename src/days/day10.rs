use crate::file_reader::read_input;
use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
struct Point {
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Point {
    fn parse(line: &str) -> Point {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)>\s?velocity=<\s*(-?\d+),\s*(-?\d+)>")
                    .unwrap();
        }
        let captures = RE.captures(&line).unwrap();
        Point {
            position_x: captures[1].parse().unwrap(),
            position_y: captures[2].parse().unwrap(),
            velocity_x: captures[3].parse().unwrap(),
            velocity_y: captures[4].parse().unwrap(),
        }
    }

    fn move_point(&mut self) {
        self.position_x += self.velocity_x;
        self.position_y += self.velocity_y;
    }
}

#[derive(Debug)]
struct MaxDims {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl MaxDims {
    fn calculate(star_field: &[Point]) -> MaxDims {
        MaxDims {
            min_x: star_field.iter().map(|x| x.position_x).min().unwrap(),
            max_x: star_field.iter().map(|x| x.position_x).max().unwrap(),
            min_y: star_field.iter().map(|x| x.position_y).min().unwrap(),
            max_y: star_field.iter().map(|x| x.position_y).max().unwrap(),
        }
    }

    fn width(&self) -> i32 {
        self.max_x - self.min_x
    }

    fn height(&self) -> i32 {
        self.max_y - self.min_y
    }
}

fn find_on_row(row: i32, star_field: &[Point]) -> HashSet<i32> {
    let ret: HashSet<i32> = HashSet::from_iter(
        star_field
            .iter()
            .filter(|x| x.position_y == row)
            .map(|x| x.position_x),
    );
    ret
}

fn print_field(star_field: &[Point]) {
    let dims = MaxDims::calculate(&star_field);

    for y in (dims.min_y - 1)..=(dims.max_y + 1) {
        let stars_on_row = find_on_row(y, &star_field);
        let mut row = Vec::new();
        for i in -1..=(dims.width() + 1) {
            if stars_on_row.contains(&(i + dims.min_x)) {
                row.push("#");
            } else {
                row.push(".");
            }
        }
        println!("{}", row.join(""));
    }
}

fn calculate_area(star_field: &[Point]) -> f64 {
    let dims = MaxDims::calculate(&star_field);
    let w_float: f64 = dims.width().into();
    let h_float: f64 = dims.height().into();
    w_float * h_float
}

pub fn run() -> (f64, u32) {
    let mut star_field = Vec::new();
    for line in read_input("input/input10.txt".to_string()) {
        star_field.push(Point::parse(&line.trim()));
    }

    let mut area = calculate_area(&star_field);
    let mut previous_star_field = star_field.clone();
    let mut seconds = 0;

    loop {
        star_field.iter_mut().for_each(|x| x.move_point());
        let new_area = calculate_area(&star_field);
        if new_area > area {
            break;
        }
        previous_star_field = star_field.clone();
        area = new_area;
        seconds += 1;
    }
    print_field(&previous_star_field);
    (area, seconds)
}
