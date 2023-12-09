use std::isize;

type Board = Vec<Vec<Objects>>;
type Path = Vec<Step>;

#[derive(PartialEq, Eq)]
enum Objects {
    Outside,
    Tile,
    Wall,
}

#[derive(PartialEq, Eq, Debug)]
enum Step {
    Move(usize),
    Left,
    Right,
}

// Determine if coordinates exist on board.
fn test_coordinates(board: &Board, (y, x): (isize, isize)) -> Option<(usize, usize)> {
    if y < 0 || x < 0 {
        return None;
    }
    let y = y as usize;
    let x = x as usize;

    if let Some(row) = board.get(y) {
        if let Some(cell) = row.get(x) {
            return Some((y, x));
        }
    }
    None
}

/// Return true if position is outside the map.  This doesn't
/// guarantee you can wrap, since it doesn't check for walls.
fn is_outside(board: &Board, (y, x): (isize, isize)) -> bool {
    if let Some((y, x)) = test_coordinates(board, (y, x)) {
        board[y][x] == Objects::Outside
    } else {
        true
    }
}

/// Try to wrap from position (y,x) given motion mvmt
fn try_wrap(board: &Board, (y, x): (isize, isize), (my, mx): (isize, isize)) -> (usize, usize) {
    // Reverse direction
    let (search_my, search_mx) = go_back((my, mx));
    let mut can_wrap = false;
    let mut dest_x: usize = 0;
    let mut dest_y: usize = 0;
    let mut seek_x = x as isize;
    let mut seek_y = y as isize;

    loop {
        seek_y += search_my;
        seek_x += search_mx;
        if let Some((test_y, test_x)) = test_coordinates(board, (seek_y, seek_x)) {
            match board[test_y][test_x] {
                Objects::Outside => {}
                Objects::Tile => {
                    dest_x = test_x;
                    dest_y = test_y;
                    can_wrap = true;
                }
                Objects::Wall => {
                    can_wrap = false;
                }
            }
        } else {
            break;
        }
    }

    if can_wrap {
        (dest_y, dest_x)
    } else {
        (y as usize, x as usize)
    }
}

// Move player from (px,py) to the next valid position, moving by
// motion (my,mx).
fn move1(
    board: &Board,
    (py, px): (usize, usize),
    mvmt @ (my, mx): (isize, isize),
) -> (usize, usize) {
    let (next_y, next_x) = (py as isize + my, px as isize + mx);
    if is_outside(board, (next_y, next_x)) {
        // println!("Wrap?");
        return try_wrap(board, (py as isize, px as isize), mvmt);
    } else if board[next_y as usize][next_x as usize] == Objects::Tile {
        // println!("Move");
        return (next_y as usize, next_x as usize);
    } else if board[next_y as usize][next_x as usize] == Objects::Wall {
        // println!("Stay");
        return (py, px);
    }
    panic!();
}

fn draw_board(board: &Board, player: Option<(usize, usize)>) {
    let mut y = 0;
    let mut x = 0;
    for line in board {
        x = 0;
        for cell in line {
            if player.is_some_and(|coords| coords == (y, x)) {
                print!("█")
            } else {
                print!(
                    "{}",
                    match cell {
                        Objects::Outside => ' ',
                        Objects::Tile => '.',
                        Objects::Wall => '#',
                    }
                );
            }
            x += 1;
        }
        println!();
        y += 1;
    }
}

fn read_input() -> (Board, Path) {
    let mut board: Board = vec![];
    let mut path: Vec<Step> = vec![];
    let mut lines = lib::read_lines("inputs/22-easy.txt")
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    for line in &lines {
        if line.is_empty() {
            break;
        }
        board.push(
            line.chars()
                .map(|c| match c {
                    ' ' => Objects::Outside,
                    '.' => Objects::Tile,
                    '#' => Objects::Wall,
                    _ => panic!(),
                })
                .collect::<Vec<Objects>>(),
        );
    }

    let mut acc: usize = 0;
    for c in lines.last().unwrap().chars() {
        if c.is_ascii_digit() {
            acc *= 10;
            acc += lib::read_digit(c) as usize;
        } else {
            path.push(Step::Move(acc));
            path.push(if c == 'L' { Step::Left } else { Step::Right });
            acc = 0;
        }
    }
    if acc > 0 {
        path.push(Step::Move(acc));
    }

    (board, path)
}

fn find_start(board: &Board) -> (usize, usize) {
    for y in 0.. {
        for x in 0.. {
            if board[y][x] == Objects::Tile {
                return (y, x);
            }
        }
    }
    panic!();
}

fn rotate_left((y, x): (isize, isize)) -> (isize, isize) {
    if x == 0 {
        (0, y)
    } else {
        (-x, 0)
    }
}

fn rotate_right((y, x): (isize, isize)) -> (isize, isize) {
    if x == 0 {
        (0, -y)
    } else {
        (x, 0)
    }
}

fn go_back((y, x): (isize, isize)) -> (isize, isize) {
    (-y, -x)
}

fn facing(mvmt: (isize,isize)) -> usize {
    match mvmt{
        (0, 1) => 0, // Right,
        (0, -1) => 2, // Left
        (1, 0) => 1, // Down
        (-1, 0) => 3, // Up
        _ => panic!(),
    }
}

fn main() {
    let (board, path) = read_input();
    let mut player = find_start(&board);
    let mut motion = (0, 1);
    draw_board(&board, Some(player));

    println!("{:?}", test_coordinates(&board, (15, 3)));

    for step in path {
        println!("Motion is {motion:?}, player at {player:?}, going {step:?}");
        match step {
            Step::Move(n) => {
                for _ in 0..n {
                    player = move1(&board, player, motion);
                }
                // draw_board(&board, Some(player));
            }
            Step::Left => motion = rotate_left(motion),
            Step::Right => motion = rotate_right(motion),
        }
    }
    println!("Player ends at {player:?}, facing {motion:?}");
    let row = player.0 + 1;
    let col = player.1 + 1;
    let facing = facing(motion);
    println!("Password is 1000*{row} + 4*{col} + {facing} = {}",  row * 1000 + col * 4 + facing);
}
