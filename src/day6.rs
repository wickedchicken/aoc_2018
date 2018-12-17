use crate::file_reader::read_input;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct BoundingBox {
    x_l: i32,
    x_h: i32,
    y_l: i32,
    y_h: i32,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn parse(line: &str) -> Coord {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+), (\d+)").unwrap();
        }
        let captures = RE.captures(&line).unwrap();

        Coord {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
        }
    }

    fn manhattan(&self, other: Coord) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

fn bounding_box(coords: &[Coord]) -> BoundingBox {
    BoundingBox {
        x_l: coords.iter().map(|x| x.x).min().unwrap(),
        x_h: coords.iter().map(|x| x.x).max().unwrap(),
        y_l: coords.iter().map(|x| x.y).min().unwrap(),
        y_h: coords.iter().map(|x| x.y).max().unwrap(),
    }
}

fn find_closest(point: Coord, coords: &[Coord]) -> Option<usize> {
    let mut distances: Vec<(usize, i32)> = coords
        .iter()
        .enumerate()
        .map(|(i, x)| (i, point.manhattan(*x)))
        .collect();
    distances.sort_unstable_by_key(|x| x.1);
    if distances.len() > 1 {
        if distances[0].1 != distances[1].1 {
            return Some(distances[0].0);
        }
        return None;
    }
    return Some(distances[0].0);
}

fn count_areas(points: &[Coord]) -> i32 {
    let bounds = bounding_box(&points);
    let mut outer_coords = HashSet::new();
    let mut areas = HashMap::new();

    // exclude top and bottom borders as infinite
    for x in bounds.x_l..(bounds.x_h + 1) {
        let lower = find_closest(
            Coord {
                x: x,
                y: bounds.y_l,
            },
            points,
        );
        if let Some(m) = lower {
            outer_coords.insert(m);
        }
        let upper = find_closest(
            Coord {
                x: x,
                y: bounds.y_h,
            },
            points,
        );
        if let Some(m) = upper {
            outer_coords.insert(m);
        }
    }
    // exclude left and right borders as infinite
    for y in bounds.y_l..(bounds.y_h + 1) {
        let left = find_closest(
            Coord {
                x: bounds.x_l,
                y: y,
            },
            points,
        );
        if let Some(m) = left {
            outer_coords.insert(m);
        }
        let right = find_closest(
            Coord {
                x: bounds.x_h,
                y: y,
            },
            points,
        );
        if let Some(m) = right {
            outer_coords.insert(m);
        }
    }

    // go through and add to hashmap eliminating infinite areas
    for x in bounds.x_l..(bounds.x_h + 1) {
        for y in bounds.y_l..(bounds.y_h + 1) {
            let closest = find_closest(Coord { x: x, y: y }, points);
            if let Some(m) = closest {
                if !outer_coords.contains(&m) {
                    let area_entry = areas.entry(m).or_insert(0);
                    *area_entry += 1;
                }
            }
        }
    }

    return *(areas.values().max().unwrap());
}

pub fn run() -> i32 {
    let mut coords = Vec::new();
    for line in read_input("input/input6.txt".to_string()) {
        coords.push(Coord::parse(line.trim()));
    }

    return count_areas(&coords);
}
