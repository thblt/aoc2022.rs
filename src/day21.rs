// Bug: because integer division is rounding, this returns results for
// part 2 that are, well, technically correct, but

use std::{collections::HashMap, fmt::Display, str::FromStr};

use lib::*;
use sscanf::sscanf;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Op {
    Add,
    Div,
    Mul,
    Sub,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Op::*;
        match s {
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "+" => Ok(Add),
            "-" => Ok(Sub),
            _ => Err(()),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Mul => write!(f, "×"),
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Div => write!(f, "/"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Monkey {
    Literal(i128),
    Ref(String),
    Op(Op, Box<Monkey>, Box<Monkey>),
    Human,
}

impl Monkey {
    /// Returns `true` if the monkey is [`Literal`].
    ///
    /// [`Literal`]: Monkey::Literal
    #[must_use]
    fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(..))
    }

    fn as_literal(&self) -> Option<&i128> {
        if let Self::Literal(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_monkey_ref(&self) -> Option<&String> {
        if let Self::Ref(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the monkey is [`Ref`].
    ///
    /// [`Ref`]: Monkey::Ref
    #[must_use]
    fn is_ref(&self) -> bool {
        matches!(self, Self::Ref(..))
    }

    /// Returns `true` if the monkey is [`Op`].
    ///
    /// [`Op`]: Monkey::Op
    #[must_use]
    fn is_op(&self) -> bool {
        matches!(self, Self::Op(..))
    }
}

fn read_input(s: &str, find_human: bool) -> HashMap<String, Monkey> {
    let mut ret = HashMap::new();

    for line in read_lines(s) {
        let line = line.unwrap();
        if let Ok((name, number)) = sscanf!(line, "{String}: {i128}") {
            if find_human && name == "humn" {
                ret.insert(name, Monkey::Human);
            } else {
                ret.insert(name, Monkey::Literal(number));
            }
        } else if let Ok((name, left, op, right)) =
            sscanf!(line, "{String}: {String} {String} {String}")
        {
            ret.insert(
                name,
                Monkey::Op(
                    op.parse().unwrap(),
                    Box::new(Monkey::Ref(left)),
                    Box::new(Monkey::Ref(right)),
                ),
            );
        }
    }
    ret
}

enum Expression {
    Literal(i128),
    Operation {
        op: Op,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Human,
}

impl Expression {
    fn new(monkeys: &HashMap<String, Monkey>, name: &str) -> Self {
        if let Some(mnk) = monkeys.get(&name.to_string()) {
            Self::new_from_monkey(monkeys, mnk)
        } else {
            panic!("No monkey bears that name.");
        }
    }

    fn new_from_monkey(monkeys: &HashMap<String, Monkey>, mnk: &Monkey) -> Self {
        use Expression::*;
        match mnk {
            Monkey::Literal(l) => Literal(*l),
            Monkey::Ref(name) => Self::new(monkeys, name),
            Monkey::Op(op, left, right) => {
                let lhs = Self::new_from_monkey(monkeys, left);
                let rhs = Self::new_from_monkey(monkeys, right);
                if let (Some(left), Some(right)) = (lhs.as_literal(), rhs.as_literal()) {
                    Literal(match op {
                        Op::Add => left + right,
                        Op::Sub => left - right,
                        Op::Div => left / right,
                        Op::Mul => left * right,
                    })
                } else {
                    Operation {
                        op: *op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
            }
            Monkey::Human => Human,
        }
    }

    fn eval(&self, human: i128) -> Option<i128> {
        match self {
            Expression::Literal(n) => Some(*n),
            Expression::Operation { op, lhs, rhs } => {
                if let (Some(lhs), Some(rhs)) = (lhs.eval(human), rhs.eval(human)) {
                    match op {
                        Op::Add => lhs.checked_add(rhs),
                        Op::Sub => lhs.checked_sub(rhs),
                        Op::Mul => lhs.checked_mul(rhs),
                        Op::Div => lhs.checked_div(rhs),
                    }
                } else {
                    None
                }
            }
            Expression::Human => Some(human),
        }
    }

    fn strict_div_eval(&self, human: i128) -> Option<i128> {
        match self {
            Expression::Literal(n) => Some(*n),
            Expression::Operation { op, lhs, rhs } => {
                if let (Some(lhs), Some(rhs)) = (lhs.strict_div_eval(human), rhs.strict_div_eval(human)) {
                    match op {
                        Op::Add => lhs.checked_add(rhs),
                        Op::Sub => lhs.checked_sub(rhs),
                        Op::Mul => lhs.checked_mul(rhs),
                        Op::Div => {
                            if Some(0) == lhs.checked_rem_euclid(rhs) {
                                lhs.checked_div(rhs)
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Expression::Human => Some(human),
        }
    }

    /// Returns `true` if the expression is [`Literal`].
    ///
    /// [`Literal`]: Expression::Literal
    #[must_use]
    fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(..))
    }

    fn as_literal(&self) -> Option<&i128> {
        if let Self::Literal(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        match self {
            Literal(n) => n.fmt(f),
            Operation { op, lhs, rhs } => write!(f, "({}{}{})", lhs, op, rhs),
            Human => write!(f, "[H]"),
        }
    }
}

fn part2() {
    let monkeys = read_input("inputs/21.txt", true);
    use std::cmp::Ordering::*;

    if let Monkey::Op(_, left, right) = &monkeys["root"] {
        let left = Expression::new(&monkeys, left.as_monkey_ref().unwrap());
        let right = Expression::new(&monkeys, right.as_monkey_ref().unwrap());

        // println!("{left}");
        println!("{right}");

        let target;
        let problem;
        if left.is_literal() {
            target = left.as_literal().unwrap();
            problem = right;
        } else if right.is_literal() {
            target = right.as_literal().unwrap();
            problem = left;
        } else {
            panic!("Oops");
        }

        let mut min = 0;
        let mut max = i128::MAX;

        let reversed = problem.eval(0) > problem.eval(1);
        let mut val;
        loop {
            val = min + ((max - min) / 2);

            let result = problem.eval(val);

            let cmp = if let Some(result) = result {
                result.cmp(&target)
            } else {
                if reversed {
                    Less
                } else {
                    Greater
                }
            };

            match cmp {
                Equal => {
                    break;
                }
                Less => {
                    if reversed {
                        max = val - 1;
                    } else {
                        min = val + 1;
                    }
                }
                Greater => {
                    if reversed {
                        min = val - 1;
                    } else {
                        max = val + 1;
                    }
                }
            }
        }

        // We may be slightly off, because of division rounding.

        let mut shift = 0;
        loop
        {
            if let Some(target) = problem.strict_div_eval(val+shift) {
                println!("Part 2: {}", val+shift);
                break;
            }
            if let Some(target) = problem.strict_div_eval(val-shift) {
                println!("Part 2: {}", val-shift);
                break
            }
            shift += 1;
        }
    }
}

fn part1() {
    let monkeys = read_input("inputs/21.txt", false);
    let root = Expression::new(&monkeys, "root");
    println!("Part 1: {}", root);
}

fn main() {
    part1();
    part2();
}
