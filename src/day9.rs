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

    // @FIXME Make a clean iterator.
    fn next(&mut self) -> Option<Dir> {
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

struct Rope {
    knots: Vec<Coord>,
}

impl Rope {
    fn new(knots: usize) -> Rope {
        Rope {
            knots: vec![(0, 0); knots],
        }
    }

    fn get_closer(head: i32, tail: i32) -> i32 {
        tail + if head > tail { 1 } else { -1 }
    }

    fn apply(&mut self, dir: Dir) {
        match dir {
            Dir::Right => self.knots[0].0 += 1,
            Dir::Up => self.knots[0].1 += 1,
            Dir::Left => self.knots[0].0 -= 1,
            Dir::Down => self.knots[0].1 -= 1,
        }

        for k in 1..self.knots.len() {
            if abs_diff(self.knots[k - 1].0, self.knots[k].0) <= 1
                && abs_diff(self.knots[k - 1].1, self.knots[k].1) <= 1
            {
                return;
            }
            if self.knots[k - 1].0 != self.knots[k].0 {
                self.knots[k].0 = Rope::get_closer(self.knots[k - 1].0, self.knots[k].0);
            }
            if self.knots[k - 1].1 != self.knots[k].1 {
                self.knots[k].1 = Rope::get_closer(self.knots[k - 1].1, self.knots[k].1);
            }
        }
    }
}

fn main() {
    let mut cmds = read_input();
    let mut rope2 = Rope::new(2);
    let mut rope10 = Rope::new(10);
    let mut coords2: HashSet<Coord> = HashSet::new();
    let mut coords10: HashSet<Coord> = HashSet::new();
    loop {
        if let Some(dir) = cmds.next() {
            rope2.apply(dir);
            coords2.insert(rope2.knots[rope2.knots.len()-1]);
            rope10.apply(dir);
            coords10.insert(rope10.knots[rope10.knots.len()-1]);

        } else {
            break;
    }
    }
    println!("Part 1: {}", coords2.len());
    println!("Part 2: {}", coords10.len());
}
