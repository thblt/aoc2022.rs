use std::collections::HashSet;

use lib::*;
use sscanf::sscanf;

type Coord = (isize, isize);

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    beacon: Coord,
    dist: usize,
}

fn manhattan(a: Coord, b: Coord) -> usize {
    (abs_diff(a.0, b.0) + abs_diff(a.1, b.1)) as usize
}

fn read_input(s: &str) -> Vec<Sensor> {
    let mut ret = vec![];
    for line in read_lines(s) {
        let line = line.unwrap();
        let (sx, sy, bx, by) = sscanf!(
            line,
            "Sensor at x={isize}, y={isize}: closest beacon is at x={isize}, y={isize}"
        )
            .unwrap();
        ret.push(Sensor {
            pos: (sx, sy),
            beacon: (bx, by),
            dist: manhattan((sx, sy), (bx, by)),
        })
    }
    ret
}

fn part1() {
    let sensors = read_input("inputs/15.txt");
    let y = 2000000; // Target row

    let mut x1 = 0;
    let mut x2 = 1;
    let mut positions: Vec<Coord> = vec![];
    loop {
        let mut p1ok = false;
        let mut p2ok = false;
        let p1 = (x1, y);
        let p2 = (x2, y);
        for s in sensors.iter() {
            if !p1ok && manhattan(p1, s.pos) <= s.dist {
                positions.push(p1);
                p1ok = true;
            }
            if !p2ok && manhattan(p2, s.pos) <= s.dist {
                positions.push(p2);
                p2ok = true;
            }

            if p1ok && p2ok {
                break;
            }
        }

        if !p1ok && !p2ok {
            break;
        } else {
            x1 -= 1;
            x2 += 1;
        }
    }

    let mut positions: HashSet<Coord> = positions.into_iter().collect();
    for s in sensors {
        positions.remove(&s.beacon);
    }
    println!("Part 1: {}", positions.len());
}

fn main() {
    part1();
}
