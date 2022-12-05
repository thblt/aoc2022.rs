use lib::*;
use sscanf::sscanf;

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

type Stack = Vec<char>;
type Stacks = Vec<Stack>;
type Instructions = Vec<Instruction>;

fn read_input(s: &str) -> (Stacks, Instructions) {
    let mut stacks: Stacks = vec![
        vec![]; 9];
    let mut instructions: Instructions = vec![];

    let mut part1 = true;
    for line in read_lines(s) {
        let line = line.unwrap();
        if line.is_empty() { // Finalize stacks, move on to part 2.
            for stk in &mut stacks {
                stk.reverse();
            }
            part1 = false;
        } else if part1 { // Read stacks
            let mut chars = line.chars();
            for (i, item) in stacks.iter_mut().enumerate() {
                let char = chars.nth(if i == 0 { 1 } else { 3 })
                    .unwrap();
                if char != ' ' {
                    item.push(char);
                }
            }
        } else { // Read instructions
            let (count, from, to) =
                sscanf!(line, "move {usize} from {usize} to {usize}")
                .unwrap();
            instructions.push(Instruction { count, from, to })
        }
    }
    (stacks, instructions)
}

fn part2() {
    let (mut stacks, instructions) = read_input("inputs/5.txt");

    for instr in instructions {
        let from = instr.from - 1;
        let idx = stacks[from].len() - instr.count;
        for _ in 0..instr.count {
            let moved = stacks[from].remove(idx);
            stacks[instr.to - 1].push(moved);
        }
    }
    println!("Part 2: {}", stacks.iter().map(|s| s.last().unwrap()).collect::<String>());
}

fn part1() {
    let (mut stacks, instructions) = read_input("inputs/5.txt");

    for instr in instructions {
        for _ in 0..instr.count {
            let crt = stacks[instr.from - 1].pop().unwrap();
            stacks[instr.to - 1].push(crt);
        }
    }

    println!("Part 1: {}",
             stacks.into_iter().map(|s| *s.last().unwrap()).collect::<String>());
}

fn main() {
    part1();
    part2();
}
