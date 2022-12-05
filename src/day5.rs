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
    let mut stacks: Stacks = vec![vec![]; 9];
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

fn main() {
    let (mut stacks1, instructions) = read_input("inputs/5.txt");
    let mut stacks2 = stacks1.clone();

    for instr in instructions {
        // Part 1
        for _ in 0..instr.count {
            let crt = stacks1[instr.from - 1].pop().unwrap();
            stacks1[instr.to - 1].push(crt);
        }

        // Part 2
        let from = instr.from - 1;
        let idx = stacks2[from].len() - instr.count;
        let mut moved = stacks2[from].drain(idx..).collect::<Vec<char>>();
        stacks2[instr.to-1].append(&mut moved);
    }
    println!("Part 1: {}\nPart 2: {}",
             stacks1.iter().map(|s| s.last().unwrap()).collect::<String>(),
             stacks2.iter().map(|s| s.last().unwrap()).collect::<String>());
}
