use std::fmt::Display;

use lib::matrix::*;
use lib::*;

type Maze = Matrix<MazePoint>;
type Coord = (isize, isize);

#[derive(Copy, Clone)]
enum Point {
    Goal,
    Start,
    Point(u8),
}

impl Point {
    fn from_char(c: char) -> Point {
        match c {
            'E' => Point::Goal,
            'S' => Point::Start,
            _ => Point::Point(c as u8 - 'a' as u8),
        }
    }

    #[must_use]
    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    #[must_use]
    fn is_goal(&self) -> bool {
        matches!(self, Self::Goal)
    }

    fn elevation(&self) -> u8 {
        match self {
            Point::Goal => 25,
            Point::Start => 0,
            Point::Point(e) => *e,
        }
    }
}

#[derive(Copy, Clone)]
struct MazePoint {
    point: Point,
    dist: Option<u32>,
    prev: Option<Coord>,
    done: bool,
    draw_path: bool,
}

impl MazePoint {
    fn from_char(c: char) -> MazePoint {
        let point = Point::from_char(c);
        MazePoint {
            point,
            prev: None,
            dist: if point.is_goal() { Some(0) } else { None },
            done: false,
            draw_path: false,
        }
    }
}

impl Display for MazePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.point {
                Point::Goal => 'E',
                Point::Start => 'S',
                Point::Point(c) => (c + 'a' as u8) as char,
            }
        )
    }
}

fn next_point(maze: &Maze) -> Option<Coord> {
    let mut ret = None;
    let mut best_dist = None;

    for (idx, pt) in maze.vec.iter().enumerate().filter(|(_, p)| !p.done) {
        if let Some(pt_dist) = pt.dist {
            if let Some(best) = best_dist {
                if pt_dist < best {
                    best_dist = Some(pt_dist);
                    ret = Some(maze.to_coords(idx));
                }
            } else {
                best_dist = Some(pt_dist);
                ret = Some(maze.to_coords(idx));
            }
        }
    }
    ret
}

fn neighbours(maze: &Maze, center: Coord) -> Vec<Coord> {
    let mut ret: Vec<Coord> = vec![];

    let elev = maze[center].point.elevation();
    for motion in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (x, y) = (center.0 + motion.0, center.1 + motion.1);
        if maze.test_coords(x, y) {
            let elev2 = maze[(x, y)].point.elevation();
            if elev2 + 1 >= elev {
                ret.push((x, y));
            }
        }
    }

    ret
}

fn dijkstra(maze: &mut Maze) {
    while let Some(point) = next_point(maze) {
        maze[point].done = true;

        for v in neighbours(maze, point) {
            let alt = maze[point].dist.unwrap() + 1;
            match maze[v].dist {
                Some(x) if x > alt => maze[v].dist = Some(alt),
                None => maze[v].dist = Some(alt),
                _ => {}
            }
        }
    }
}

fn read_input(s: &str) -> Maze {
    Matrix::from_vecs(
        read_lines(s)
            .map(|line| line.unwrap().chars().map(MazePoint::from_char).collect())
            .collect::<Vec<Vec<MazePoint>>>(),
    )
}

fn main() {
    let mut maze = read_input("inputs/12.txt");
    dijkstra(&mut maze);

    let mut best_dist = None;
    for (idx, m) in maze.vec.iter().enumerate() {
        if m.point.is_start() {
            println!("Part 1: {}", m.dist.unwrap());
        } else if m.point.elevation() == 0 && m.dist.is_some() {
            let dist = m.dist.unwrap();
            match best_dist {
                Some(best) if dist < best => best_dist = Some(dist),
                None => best_dist = Some(dist),
                _ => {}
            }
            if dist == 13 {
                println!("Coords: {:?}", maze.to_coords(idx))
            }
        }
    }
    println!("Part 2: {}", best_dist.unwrap());

    let mut point = (106, 20);
    loop {
        maze[point].draw_path = true;
        if let Some(next) = maze[point].prev {
            point = next;
        } else {
            break;
        }
    }
    println!("{}", maze);
}
