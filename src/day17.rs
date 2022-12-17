use lib::matrix::*;
use lib::*;

type Shape = Matrix<bool>;

#[derive(Clone)]
struct Chamber {
    chamber: Matrix<bool>,
    ceiling: isize,
}

impl Chamber {
    fn new(height: usize) -> Chamber {
        Chamber {
            chamber: Matrix::new_default(7, height, false),
            ceiling: height as isize,
        }
    }

    fn add_shape(&mut self, shape: &Matrix<bool>, x: usize, y: usize) {
        for y2 in 0..shape.height() as isize {
            for x2 in 0..shape.width() as isize {
                if shape[(x2, y2)] {
                    let y = y2 + y as isize;
                    self.chamber[(x2 + x as isize, y)] = true;
                    if y < self.ceiling {
                        self.ceiling = y;
                    }
                }

            }
        }
    }

    fn can_fit(&self, shape: &Matrix<bool>, x: usize, y: usize) -> bool {
        if x + shape.width() > self.chamber.width() || y + shape.height() > self.chamber.height() {
            return false;
        }
        for y2 in 0..shape.height() as isize {
            for x2 in 0..shape.width() as isize {
                if self.chamber[(x2 + x as isize, y2 + y as isize)] && shape[(x2, y2)] {
                    return false;
                }
            }
        }
        true
    }

    fn find_ceiling(&mut self) {
        // for y in 0..self.chamber.height() as isize {
        //     for x in 0..self.chamber.width() as isize {
        //         if self.chamber[(x, y)] {
        //             return self.ceiling = y;
        //         }
        //     }
        // }
    }

    fn draw(&self) {
        let mut repr = String::new();
        repr += "┏━━━━━━━┓\n";
        for y in self.ceiling - 3..self.chamber.height() as isize {
            repr += "┃";
            for x in 0..self.chamber.width() as isize {
                repr += if self.chamber[(x, y)] { "#" } else { "." };
            }
            repr += "┃\n";
        }
        repr += "┗━━━━━━━┛";
        println!("{repr}");
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
    let mut chamber: Chamber = Chamber::new(4000);

    for _ in 1..=2022 {
        // Shapes
        let shape = shapes.next().unwrap();
        let mut x: isize = 2;
        let mut y = chamber.ceiling as usize - shape.height() - 3;
        loop {
            // Descent
            let jet = jets.next().unwrap();
            let new_x: isize = x + jet;
            if new_x >= 0 && chamber.can_fit(&shape, new_x as usize, y) {
                x = new_x;
            }

            let new_y = y + 1;
            if chamber.can_fit(&shape, x as usize, new_y) {
                y = new_y;
            } else {
                chamber.add_shape(&shape, x as usize, y);
                break;
            }
        }
        chamber.find_ceiling();
        // println!("After rock {:4}, height is {:4}", i, chamber.chamber.height() - chamber.ceiling as usize);
    }

    // chamber.draw();

    println!(
        "Height: {}",
        chamber.chamber.height() - chamber.ceiling as usize
    );
}
