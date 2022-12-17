use lib::*;
use sscanf::sscanf;
use std::str::FromStr;

type MonkeyId = usize;
type Item = u64;
type Level = u64;
type Monkeys = Vec<Monkey>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Op {
    Add,
    Mul,
    Pow,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Monkey {
    items: Vec<Item>,
    op: Op,
    op_constant: Level,
    div_test: Level,
    throw_to: (MonkeyId, MonkeyId),
    counter: u64,
}

fn read_input(path: &str) -> Monkeys {
    let mut lines = read_lines(path).map(|l| l.unwrap());
    let mut monkeys: Monkeys = Monkeys::new();
    loop {
        let _line = lines.next().unwrap();
        // let id = sscanf!(line, "Monkey {MonkeyId}:").unwrap();

        let line = lines.next().unwrap();
        let items = sscanf!(line, "  Starting items: {String}").unwrap();

        let line = lines.next().unwrap();
        let (op, c) = sscanf!(line, "  Operation: new = old {String} {String}").unwrap();
        let mut op = op.parse::<Op>().unwrap();

        let op_constant: Level;
        if let Ok(c) = c.parse::<Level>() {
            op_constant = c;
        } else {
            op = Op::Pow;
            op_constant = 2;
        };

        let line = lines.next().unwrap();
        let div_test = sscanf!(line, "  Test: divisible by {Level}").unwrap();

        let line1 = lines.next().unwrap();
        let line2 = lines.next().unwrap();
        let throw_to = (
            sscanf!(line1, "    If true: throw to monkey {MonkeyId}").unwrap(),
            sscanf!(line2, "    If false: throw to monkey {MonkeyId}").unwrap(),
        );

        let monkey = Monkey {
            // id,
            items: items
                .split(", ")
                .map(|c| c.parse::<Item>().unwrap())
                .collect::<Vec<Item>>(),
            op,
            op_constant,
            div_test,
            throw_to,
            counter: 0,
        };
        // println!("{:?}", &monkey);
        monkeys.push(monkey);
        if lines.next().is_none() {
            break;
        }
    }
    monkeys
}

fn main() {
    let mut monkeys = read_input("inputs/11.txt");
    let mut throws: Vec<Vec<Item>> = monkeys.iter().map(|_| vec![]).collect();

    let divs: u64 = monkeys.iter().map(|m| m.div_test).product();

    for _ in 0..10000 {
        for (id, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.append(&mut throws[id]);
            for object in monkey.items.drain(0..monkey.items.len()) {
                // println!("Monkey {} inspects an item with worry level {}", id, object);
                monkey.counter += 1;
                // Worry level changes during inspection.
                let mut level = match monkey.op {
                    Op::Add => object + monkey.op_constant,
                    Op::Mul => object * monkey.op_constant,
                    Op::Pow => object.pow(monkey.op_constant as u32),
                };

                // PART 1:
                // level /= 3;

                level %= divs;

                throws[if level % monkey.div_test == 0 {
                    monkey.throw_to.0
                } else {
                    monkey.throw_to.1
                }]
                    .push(level);
            }
        }
    }
    let mut counts = monkeys.iter().map(|m| m.counter).collect::<Vec<u64>>();
    counts.sort_by(|a, b| b.cmp(a));
    println!("Part 1: see comment in code.");
    // TO RESTORE PART 1:
    //  - Bring loop back to 20 iterations.
    //  - Restore `level /= 3`;
    println!(
        "Part 2: {}×{}={}",
        counts[0],
        counts[1],
        counts[0] * counts[1]
    );
}
