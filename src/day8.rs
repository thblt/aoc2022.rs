use lib::matrix::*;
use lib::*;
use std::collections::HashSet;

fn scenic_score(trees: &Matrix<i8>, x: isize, y: isize) -> u64 {
    let mut counts: (u64, u64, u64, u64) = (0, 0, 0, 0);
    let mut done = (false, false, false, false);
    let max_height = trees[(x, y)];
    for d in 1..trees.width() as isize {
        for (state, counter, dx, dy) in [(&mut done.0, &mut counts.0, -d, 0),
                                         (&mut done.1, &mut counts.1, d, 0),
                                         (&mut done.2, &mut counts.2, 0, -d),
                                         (&mut done.3, &mut counts.3, 0, d)] {

            if !*state {
                if let Some(tree) = trees.get(x+dx, y+dy) {
                    if tree >= max_height {
                        *counter = d as u64;
                        *state = true;
                    }
                } else {
                    *counter = (d - 1) as u64;
                    *state = true;
                }
            }
        }

        if done == (true, true, true, true) {
            break;
        }
    }
    counts.0 * counts.1 * counts.2 * counts.3
}

fn part2() {
    let trees = read_forest();
    let mut best = 0;
    let mut best_coords = (0, 0);
    for x in 0..trees.width() as isize {
        for y in 0..trees.height() as isize {
            let ss = scenic_score(&trees, x, y);
            if ss > best {
                best = ss;
                best_coords = (x, y)
            }
        }
    }
    println!("Part 2: {} (at {:?})", best, best_coords);
}

fn read_forest() -> Matrix<i8> {
    Matrix::from_vecs(
        read_lines("inputs/8.txt")
            .map(|c| {
                c.unwrap()
                    .chars()
                    .map(|c| read_digit(c) as i8)
                    .collect::<Vec<i8>>()
            })
            .collect::<Vec<Vec<i8>>>(),
    )
}

fn part1() {
    let trees = read_forest();
    println!("That forest is {}×{}.", trees.width(), trees.height());

    let dim = (trees.width() - 1) as isize;
        let mut visible: HashSet<(isize, isize)> = HashSet::new();

        for a in 0..=dim {
            let mut thresholds = (-1, -1, -1, -1);
            for b in 0..=dim {
                // Left -> right
                if trees[(a, b)] > thresholds.0 {
                    visible.insert((a, b));
                    thresholds.0 = trees[(a, b)]
                };
                // Right -> left
                if trees[(a, dim - b)] > thresholds.1 {
                    visible.insert((a, dim - b));
                    thresholds.1 = trees[(a, dim - b)]
                };
                // Top -> Bottom
                if trees[(b, a)] > thresholds.2 {
                    visible.insert((b, a));
                    thresholds.2 = trees[(b, a)];
                };
                if trees[(dim - b, a)] > thresholds.3 {
                    thresholds.3 = trees[(dim - b, a)];
                    visible.insert((dim - b, a));
                }
            }
            // Right -> left
    }

    println!("Part 1: {:?}", visible.len());
}

fn main() {
    part1();
    part2();
}
