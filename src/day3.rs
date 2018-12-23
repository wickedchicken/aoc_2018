use crate::file_reader::read_input;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Claim {
    id: u32,
    posx: i32,
    posy: i32,
    lenx: i32,
    leny: i32,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

pub fn run() -> (usize, u32) {
    let mut claims = Vec::new();
    let re = Regex::new(r"#(\d+) @ (-?\d+),(-?\d+): (\d+)x(\d+)").unwrap();
    for line in read_input("input/input3.txt".to_string()) {
        let caps = re.captures(line.trim()).unwrap();
        claims.push(Claim {
            id: (&caps[1]).parse().unwrap(),
            posx: (&caps[2]).parse().unwrap(),
            posy: (&caps[3]).parse().unwrap(),
            lenx: (&caps[4]).parse().unwrap(),
            leny: (&caps[5]).parse().unwrap(),
        });
    }

    let mut grid = HashMap::new();
    let mut collisions = HashSet::new();
    let mut overlapping_claims = HashSet::new();

    for claim in claims.iter() {
        for xval in claim.posx..(claim.posx + claim.lenx) {
            for yval in claim.posy..(claim.posy + claim.leny) {
                let coord = Coord { x: xval, y: yval };
                if grid.contains_key(&coord) {
                    collisions.insert(coord);
                    overlapping_claims.insert(claim.id);
                    overlapping_claims.extend(&grid[&coord]);
                }
                let grid_set = grid.entry(coord).or_insert_with(HashSet::new);
                grid_set.insert(claim.id);
            }
        }
    }

    let key_set = HashSet::from_iter(claims.iter().map(|x| x.id));
    let non_overlapping: Vec<&u32> = key_set.difference(&overlapping_claims).collect();
    assert_eq!(non_overlapping.len(), 1);

    (collisions.len(), *non_overlapping[0])
}
