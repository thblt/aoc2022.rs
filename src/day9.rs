use std::collections::HashSet;
use std::str::FromStr;

use lib::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Dir {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir::Right),
            "L" => Ok(Dir::Left),
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            _ => Err(()),
        }
    }
}

type Command = (Dir, u32);

fn read_command(s: &str) -> Command {
    let s = s.to_string();
    let mut sides = s.split(' ');
    (
        sides.next().unwrap().parse::<Dir>().unwrap(),
        sides.next().unwrap().parse::<u32>().unwrap(),
    )
}

#[derive(PartialEq, Eq, Clone)]
struct CommandStream {
    idx: usize,
    rem: u32,
    stream: Vec<Command>,
}

fn read_input() -> CommandStream {
    CommandStream::new(
        &read_lines("inputs/9.txt")
            .map(|l| read_command(&l.unwrap()))
            .collect::<Vec<Command>>(),
    )
}

impl CommandStream {
    fn new(cs: &[Command]) -> Self {
        Self {
            idx: 0,
            rem: cs[0].1,
            stream: cs.to_vec(),
        }
    }
}

impl Iterator for CommandStream {
    type Item = Dir;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem > 0 {
            self.rem -= 1;
            Some(self.stream[self.idx].0)
        } else if self.idx < self.stream.len() - 1 {
            self.idx += 1;
            self.rem = self.stream[self.idx].1;
            self.next()
        } else {
            None
        }
    }
}

type Coord = (i32, i32);

struct Rope (Vec<Coord>);

impl Rope {
    fn new(knots: usize) -> Rope {
        Rope(vec![(0, 0); knots])
    }

    fn get_closer(head: i32, tail: i32) -> i32 {
        tail + if head > tail { 1 } else { -1 }
    }

    fn apply(&mut self, dir: Dir) {
        use Dir::*;
        match dir {
            Right => self.0[0].0 += 1,
            Up => self.0[0].1 += 1,
            Left => self.0[0].0 -= 1,
            Down => self.0[0].1 -= 1,
        }

        for k in 1..self.0.len() {
            if abs_diff(self.0[k - 1].0, self.0[k].0) <= 1
                && abs_diff(self.0[k - 1].1, self.0[k].1) <= 1
            {
                return;
            }
            if self.0[k - 1].0 != self.0[k].0 {
                self.0[k].0 = Rope::get_closer(self.0[k - 1].0, self.0[k].0);
            }
            if self.0[k - 1].1 != self.0[k].1 {
                self.0[k].1 = Rope::get_closer(self.0[k - 1].1, self.0[k].1);
            }
        }
    }
}

fn main() {
    let mut rope2 = Rope::new(2);
    let mut rope10 = Rope::new(10);
    let mut coords2: HashSet<Coord> = HashSet::new();
    let mut coords10: HashSet<Coord> = HashSet::new();
    for dir in read_input() {
        rope2.apply(dir);
        rope10.apply(dir);
        coords2.insert(*rope2.0.last().unwrap());
        coords10.insert(*rope10.0.last().unwrap());
    }
    println!("Part 1: {}", coords2.len());
    println!("Part 2: {}", coords10.len());
}
