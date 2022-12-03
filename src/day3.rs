#![warn(clippy::pedantic)]

use lib::read_lines;
use std::collections::HashSet;

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
}

fn part2() {
    let mut total = 0;
    let mut collector: Vec<HashSet<char>> = vec!();
    for line in read_lines("inputs/3.txt") {
        if collector.len() < 2 {
            collector.push(line.unwrap().chars().collect::<HashSet<_>>());
        } else {
            collector.push(line.unwrap().chars().collect::<HashSet<_>>());
            let badge = collector[0]
                    .intersection(&collector[1])
                    .copied()
                    .collect::<HashSet<char>>()
                    .intersection(&collector[2])
                .copied()
                .collect::<Vec<char>>()
                [0];
            total += priority(badge);
            collector.clear();
        }
    }
    println!("Part 2: {}", total);
}

fn part1() {
    let mut total = 0;
    for line in read_lines("inputs/3.txt") {
        let line: Vec<char> = line.unwrap().chars().collect();
        let left: HashSet<char> = line[0..line.len()/2].iter().copied().collect::<HashSet<_>>();
        let right: HashSet<char> = line[line.len()/2..].iter().copied().collect::<HashSet<_>>();
        let diff = left.intersection(&right);
        for c in diff {
            total += priority(*c);
        }
    }
    println!("Part 1: {}", total);
}

fn main() {
    part1();
    part2();
}
