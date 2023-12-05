use std::ops::{Index, IndexMut};

// Part 2 is still unsolved.  Solution is probably not to compute the
// whole tower, but to look for cycle.  We could look for cycles by
// using a few simple values (shape index, jets index at beginning,
// steps of fall, total height increase…)  and confirm after.

use lib::matrix::*;
use lib::*;

const CHAMBER_HEIGHT: isize = 100000000;
const CHAMBER_WIDTH: isize = 7;
const RING_ROTATION_THRESHOLD: isize = 50;

type Shape = Matrix<bool>;

// The chamber in which the rocks fall is represented as a ring. This
// is because we don't really need to accumulate the total height of
// 1000000000000 rocks.  Every few ten rocks, a row will end up fully
// filled, like so:

// |..##...|
// |..#.#..|
// |..#.#..|
// |#######| <<<<<
// |..####.|
// |...#...|
// |..####.|
//
// We can then set the zero of our ring at this row, and just store
// the number of rows that were below it.
//
// For simplicity, coordinates start at the lower-left corner:
// 2,0 |..####.| 6,2
//     |...#...|
// 0,0 |..####.| 6,0
//     \-------/

#[derive(Clone)]
struct Chamber {
    // The chamber contents
    vec: Vec<bool>,
    // Ceiling position
    ceiling: isize,
    // zero-y position
    y_shift: isize,
    // count of ignored blocks "under zero".
    removed_height: isize,
}

impl Chamber {
    pub fn to_index(&self, (x, mut y): (isize, isize)) -> usize {
        if x >= CHAMBER_WIDTH {
            panic!("Bad x");
        }

        if y >= CHAMBER_HEIGHT {
            panic!("Bad y");
        }

        y += self.y_shift;
        y %= CHAMBER_HEIGHT;

        let ret = ((y * CHAMBER_WIDTH + x) as usize) % self.vec.len();
        ret
    }

    fn new() -> Chamber {
        Chamber {
            vec: [false; (CHAMBER_HEIGHT * 7) as usize].to_vec(),
            ceiling: -1,
            y_shift: 0,
            removed_height: 0,
        }
    }

    /// Shift the y zero to a new value relative to the current y
    /// zero.
    fn shift_y(&mut self, y_shift: isize) {
        // println!("Rotating to {y_shift}, was {}, ceiling was {}, result is {}", self.y_shift, self.ceiling, self.result());
        // Shifting
        self.y_shift = (self.y_shift + y_shift) % CHAMBER_HEIGHT;
        self.removed_height += y_shift;
        self.ceiling -= y_shift;

        // Clean from just above the ceiling
        let start = self.to_index((0, self.ceiling + 1)) as usize;
        // To base_y
        let end = start + (CHAMBER_WIDTH*(CHAMBER_HEIGHT - self.ceiling - 1)) as usize;

        // println!(
        //     "Erasing from row #{} {start}..{end} (total {})",
        //     self.ceiling + 1,
        //     end - start
        // );
        let len = self.vec.len();
        for idx in start..end{
            self.vec[idx % len] = false;
        }
        // println!("Done and cleaned up from {start} to {end}, shift_y is now {}, ceiling is {} and result has become {}", self.y_shift, self.ceiling, self.result());
    }

    fn add_shape(&mut self, shape: &Matrix<bool>, x: isize, y: isize) {
        for sy in 0..shape.height() as isize {
            for sx in 0..shape.width() as isize {
                if shape[(sx, sy)] {
                    let y = y - sy as isize;
                    self[(sx + x, y)] = true;
                    if y > self.ceiling {
                        self.ceiling = y;
                    }
                }
            }
        }
        // Determine if it's time to optimize.
        if self.ceiling > CHAMBER_HEIGHT - RING_ROTATION_THRESHOLD {
            for y in (0..self.ceiling).rev() {
                let mut filled = true;
                for x in 0..7 {
                    filled &= self[(x, y as isize)];
                }
                if filled {
                    self.shift_y(y+1);
                    break;
                }
            }
        }
        if self.ceiling > CHAMBER_HEIGHT - 10 {
            panic!("Ceiling still too low");
        }
    }

    fn can_fit(&self, shape: &Matrix<bool>, x: isize, y: isize) -> bool {
        if x + (shape.width() as isize) > CHAMBER_WIDTH || y + (shape.height() as isize) < 0 {
            return false;
        }
        for sy in 0..shape.height() as isize {
            for sx in 0..shape.width() as isize {
                if self[(x + sx as isize, y - sy as isize)] && shape[(sx, sy)] {
                    return false;
                }
            }
        }
        true
    }

    fn find_ceiling(&mut self) {
        for y in 0..CHAMBER_HEIGHT as isize {
            for x in 0..CHAMBER_WIDTH as isize {
                if self[(x, y)] {
                    return self.ceiling = y;
                }
            }
        }
    }

    // /// Limit the memory occupation of the chamber by removing all
    // /// rows <= y.  This is called by `add_piece`
    // fn reduce(&mut self, base_y: isize) {
    //     let height = CHAMBER_HEIGHT as isize;
    //     let removed_height = height - base_y;

    //     self.draw();
    //     println!("Cutting at {base_y}, height is {height}, preserving {removed_height}");

    //     // Copy
    //     for y in (0..base_y).rev() {
    //         // println!("At {y} of {height} base is {base_y}");
    //         for x in 0..7 {
    //             self[(x, y + removed_height)] = self[(x, y)];
    //         }
    //     }

    //     // print!("Old values: c={} s={} ... ", self.ceiling, self.shift);
    //     self.ceiling += removed_height;
    //     self.removed_height += removed_height as isize;
    //     // println!("NEW values: c={} s={} ... ", self.ceiling, self.shift);
    // }

    fn result(&self) -> isize {
        self.ceiling + self.removed_height + 1
    }

    fn draw(&self) {
        fn bc(b: bool) -> char {
            if b {
                '#'
            } else {
                '.'
            }
        }

        let mut repr: Vec<String> = vec![];

        let mut empty_count = 0;
        for y in 0..CHAMBER_HEIGHT {
            repr.push(format!(
                "{:10} ┃{}{}{}{}{}{}{}┃\n",
                y + self.removed_height,
                bc(self[(0, y)]),
                bc(self[(1, y)]),
                bc(self[(2, y)]),
                bc(self[(3, y)]),
                bc(self[(4, y)]),
                bc(self[(5, y)]),
                bc(self[(6, y)]),
            ));
            if !self[(0, y)]
                && !self[(1, y)]
                && !self[(2, y)]
                && !self[(3, y)]
                && !self[(4, y)]
                && !self[(5, y)]
                && !self[(6, y)]
            {
                if empty_count == 5 {
                    break;
                } else {
                    empty_count += 1
                }
            } else {
                empty_count = 0;
            }
        }
        println!("           ┏━━━━━━━┓");
        print!("{}", repr.into_iter().rev().collect::<String>());
        println!("           ┗━━━━━━━┛");
    }
}

impl Index<(isize, isize)> for Chamber {
    type Output = bool;

    fn index(&self, coords: (isize, isize)) -> &Self::Output {
        &self.vec[self.to_index(coords)]
    }
}

impl IndexMut<(isize, isize)> for Chamber {
    fn index_mut(&mut self, coords: (isize, isize)) -> &mut Self::Output {
        let i = self.to_index(coords);
        &mut self.vec[i]
    }
}

fn read_input(s: &str) -> Vec<isize> {
    read_lines(s)
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("Bad input"),
        })
        .collect()
}

fn make_shapes() -> impl Iterator<Item = Shape> {
    let mut cross = Matrix::new_default(3, 3, true);
    cross[(0, 0)] = false;
    cross[(0, 2)] = false;
    cross[(2, 0)] = false;
    cross[(2, 2)] = false;

    let mut l = Matrix::new_default(3, 3, true);
    l[(0, 0)] = false;
    l[(0, 1)] = false;
    l[(1, 0)] = false;
    l[(1, 1)] = false;

    let shapes: Vec<Shape> = vec![
        Matrix::new_default(4, 1, true),
        cross.clone(),
        l.clone(),
        Matrix::new_default(1, 4, true),
        Matrix::new_default(2, 2, true),
    ];

    shapes.into_iter().cycle()
}

fn main() {
    let mut jets = read_input("inputs/17.txt").into_iter().cycle();
    let mut shapes = make_shapes();
    let mut chamber: Chamber = Chamber::new();

    // for counter in 1..=2022_i64 {
    for counter in 1..=50000_i64 {
        // Shapes
        let shape = shapes.next().unwrap();
        let mut x: isize = 2;
        let mut y = chamber.ceiling + shape.height() as isize + 3;
        loop {
            // Descent
            let jet = jets.next().unwrap();
            let new_x = x + jet;
            if new_x >= 0 && chamber.can_fit(&shape, new_x as isize, y) {
                x = new_x;
            }

            // let mut show = chamber.clone();
            // show.add_shape(&shape, x as isize, y);
            // show.draw();

            let new_y = y - 1;
            if new_y >= 0 && chamber.can_fit(&shape, x as isize, new_y) {
                y = new_y;
            } else {
                chamber.add_shape(&shape, x as isize, y);
                break;
            }
        }
        if counter % 10000000 == 0 {
            println!("At {counter}\n  /1000000000000\n-------");
        } else if counter == 2022 {
            println!("Part 1: {}", chamber.result());
        }
    }
    chamber.draw();
    println!("Part 2: {}", chamber.result());
}
