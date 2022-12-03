use lib::*;

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Victory,
    Defeat,
    Draw,
}


fn read_move(c: &char) -> Move {
    use Move::*;
    match c {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        'X' => Rock,
        'Y' => Paper,
        'Z' => Scissors,
        _ => panic!("You played Spock"),
    }
}

fn read_outcome(c: &char) -> Outcome {
    use Outcome::*;
    match c {
        'X' => Defeat,
        'Y' => Draw,
        'Z' => Victory,
        _ => panic!("You played Spock"),
    }
}

fn pick_move(their: &Move, outcome: &Outcome) -> Move {
    use Move::*;
    use Outcome::*;
    match (their,outcome) {
        (Rock, Victory) => Paper,
        (Rock, Defeat) => Scissors,
        (Paper, Victory) => Scissors,
        (Paper, Defeat) => Rock,
        (Scissors, Victory) => Rock,
        (Scissors, Defeat) => Paper,
        (x, Draw) => *x,
    }
}

fn move_value(m: &Move)-> isize {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn score(us: &Move, them: &Move) -> (isize,isize) {
    use Move::*;
    let result = match (us, them) {
        (Rock, Paper) => (0, 6),
        (Rock, Scissors) => (6,0),
        (Paper, Rock) => (6,0),
        (Paper, Scissors) => (0,6),
        (Scissors, Rock)=> (0,6),
        (Scissors, Paper) => (6,0),
        _ => (3,3),
    };
    (move_value(us) + result.0, move_value(them) + result.1)
}

fn main() {
    let mut my_score1 = 0;
    let mut my_score2 = 0;
    for line in read_lines("inputs/2.txt") {
        let line = line.unwrap();
        let their = read_move(&line.chars().next().unwrap());
        let ours = read_move(&line.chars().nth(2).unwrap());
        let outcome = read_outcome(&line.chars().nth(2).unwrap());
        let ours2 = pick_move(&their, &outcome);
        my_score1 += score(&ours, &their).0;
        my_score2 += score(&ours2, &their).0;
    }

    println!("Score 1: {}", my_score1);
    println!("Score 2: {}", my_score2);

}
