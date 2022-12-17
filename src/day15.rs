use std::collections::HashSet;

use lib::*;
use sscanf::sscanf;

type Coord = (isize, isize);

#[derive(Debug, Hash)]
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

fn min_x(us: Coord, sensor: &Sensor) -> isize {
    let dist_y = abs_diff(sensor.pos.1, us.1);
    sensor.pos.0 - (sensor.dist - dist_y as usize) as isize
}

fn max_x(us: Coord, sensor: &Sensor) -> isize {
    let dist_y = abs_diff(sensor.pos.1, us.1);
    sensor.pos.0 + (sensor.dist - dist_y as usize) as isize
}

fn part2(sensors: &[Sensor], max: isize) {
    // We need to find the only position in (0,0) - (4000000,4000000)
    // where a beacon could be.  It's cheap, but not too cheap.  The
    // best way to simplify the search is, whenever we "hit" a sensor,
    // to jump to the other side.
    let mut x = 0;
    let mut y = 0;
    loop {
        let mut new_x = None;
        for s in sensors {
            if manhattan(s.pos, (x, y)) <= s.dist {
                let max_x = max_x((x, y), s);
                if max_x >= x {
                    new_x = Some(max_x + 1);
                    //                 ^^^ +1 because max_x is the
                    // last point *in range*.
                } else {
                    new_x = Some(x + 1);
                }
                break;
            }
        }
        if let Some(new_x) = new_x {
            x = new_x
        } else {
            println!("Part 2: Point is {},{}, freq={}", x, y, x * 4_000_000 + y);
            return;
        }

        if x > max {
            x = 0;
            y += 1;
        }
        if y > max {
            break;
        }
    }
}

fn part1(sensors: &[Sensor], y: isize) {
    let mut x1 = 0;
    let mut x2 = 1;
    let mut count: usize = 0;
    loop {
        let mut p1ok = false;
        let mut p2ok = false;
        let p1 = (x1, y);
        let p2 = (x2, y);
        for s in sensors {
            if !p1ok && manhattan(p1, s.pos) <= s.dist {
                count += 1;
                let new_x1 = min_x((x1, y), s);
                count += abs_diff(new_x1, x1) as usize;
                x1 = new_x1;
                p1ok = true;
            }
            if !p2ok && manhattan(p2, s.pos) <= s.dist {
                count += 1;
                let new_x2 = max_x((x2, y), s);
                count += abs_diff(new_x2, x2) as usize;
                x2 = new_x2;
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

    // We need to count beacons on the line, because the solution is
    // the number of "positions where a beacon cannot be present",
    // which obviously doesn't include positions where a beacon *is*.
    let beacons: usize = sensors
        .iter()
        .map(|s| s.beacon)
        .filter(|b| b.1 == y)
        .collect::<HashSet<Coord>>()
        .len();

    println!("Part 1: {}", count - beacons);
}

fn main() {
    let sensors = read_input("inputs/15.txt");

    part1(&sensors, 2000000);
    part2(&sensors, 4_000_000);
}
