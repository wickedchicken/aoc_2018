use crate::file_reader::read_input;
use regex::Regex;
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

#[derive(Debug)]
struct MaxDims {
    minx: i32,
    miny: i32,
    width: usize,
    height: usize,
}

fn dims(claims: &[Claim]) -> MaxDims {
    let minx = claims.iter().map(|x| x.posx).min().unwrap();
    let miny = claims.iter().map(|x| x.posy).min().unwrap();
    MaxDims {
        minx,
        miny,
        width: (claims.iter().map(|x| x.posx + x.lenx).max().unwrap() - minx) as usize,
        height: (claims.iter().map(|x| x.posy + x.leny).max().unwrap() - miny) as usize,
    }
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

    let max_dims = dims(&claims);

    let mut grid = Vec::with_capacity(max_dims.height);
    for _ in 0..max_dims.height {
        let mut row = Vec::with_capacity(max_dims.width);
        for _ in 0..max_dims.width {
            row.push(HashSet::new());
        }
        grid.push(row);
    }

    let mut collisions = HashSet::new();
    let mut overlapping_claims = HashSet::new();

    for claim in claims.iter() {
        let xoffset = claim.posx - max_dims.minx;
        let yoffset = claim.posy - max_dims.miny;

        for yval in yoffset..(yoffset + claim.leny) {
            for xval in xoffset..(xoffset + claim.lenx) {
                let coord = Coord { x: xval, y: yval };
                let grid_set = &mut grid[yval as usize][xval as usize];
                if !grid_set.is_empty() {
                    collisions.insert(coord);
                    overlapping_claims.insert(claim.id);
                    overlapping_claims.extend(grid_set.iter());
                }
                grid_set.insert(claim.id);
            }
        }
    }

    let key_set = HashSet::from_iter(claims.iter().map(|x| x.id));
    let non_overlapping: Vec<&u32> = key_set.difference(&overlapping_claims).collect();
    assert_eq!(non_overlapping.len(), 1);

    (collisions.len(), *non_overlapping[0])
}
