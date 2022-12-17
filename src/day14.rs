use std::fmt::Display;

use lib::matrix::*;
use lib::*;
use sscanf::sscanf;

type Coord = (isize, isize);

#[derive(Clone)]
struct Cave {
    cave: Matrix<Element>,
    floor: isize,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Element {
    Sand,
    Air,
    Rock,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::Sand => "o",
                Element::Air => ".",
                Element::Rock => "#",
            }
        )
    }
}

fn read_input(path: &str) -> Cave {
    let mut cave = Matrix::new_default(1000, 1000, Element::Air);
    let mut floor: isize = 0;

    for line in read_lines(path) {
        let line = line.unwrap();
        let mut start: Option<Coord> = None;
        for pair in line.split(" -> ") {
            let end: Coord = sscanf!(pair, "{isize},{isize}").unwrap();
            // Update floor
            if end.1 > floor - 2 {
                floor = end.1 + 2;
            }

            if let Some(start) = start {
                let constant;
                let range;
                let swap;
                if start.0 == end.0 {
                    constant = start.0;
                    range = std::cmp::min(start.1, end.1)..std::cmp::max(start.1, end.1) + 1;
                    swap = false;
                } else if start.1 == end.1 {
                    constant = start.1;
                    range = std::cmp::min(start.0, end.0)..std::cmp::max(start.0, end.0) + 1;
                    swap = true;
                } else {
                    panic!("Bad input");
                }

                for var in range {
                    if swap {
                        cave[(var, constant)] = Element::Rock;
                    } else {
                        cave[(constant, var)] = Element::Rock;
                    }
                }
            }
            // Proceed
            start = Some(end);
        }
    }
    Cave { cave, floor }
}

fn add_sand(cave: &mut Cave) -> bool {
    let mut pos: Coord = (500, 0);
    if cave.cave[pos] != Element::Air {
        return false;
    }

    'outer: loop {
        for cand in [
            (pos.0, pos.1 + 1),
            (pos.0 - 1, pos.1 + 1),
            (pos.0 + 1, pos.1 + 1),
        ] {
            if cand.1 < cave.floor && cave.cave.get(cand.0, cand.1) == Some(Element::Air) {
                pos = cand;
                continue 'outer;
            }
        }
        // No candidate
        if cave.cave.test_coords(pos.0, pos.1 + 1) {
            cave.cave[(pos)] = Element::Sand;
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let mut cave1 = read_input("inputs/14.txt");
    let mut cave2 = cave1.clone();

    cave1.floor = 2000;
    let mut counter = 0;
    while add_sand(&mut cave1) {
        counter += 1;
    }
    println!("Part 1: {}", counter);

    let mut counter = 0;
    while add_sand(&mut cave2) {
        counter += 1;
    }
    println!("Part 2: {}", counter);
}
