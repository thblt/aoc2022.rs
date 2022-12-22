use std::{collections::HashMap, str::FromStr};

use lib::*;
use sscanf::sscanf;

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
enum Op {
    Add, Div, Mul, Sub
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
            _ => Err(())
        }
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
enum Monkey {
    Literal(i128),
    Ref(String),
    Op(Op, Box<Monkey>, Box<Monkey>)
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
}

fn read_input(s: &str) -> HashMap<String,Monkey> {
    let mut ret = HashMap::new();

    for line in read_lines(s) {
        let line = line.unwrap();
        if let Ok((name, number)) = sscanf!(line, "{String}: {i128}") {
            ret.insert(name, Monkey::Literal(number));
        } else if let Ok((name, left, op, right)) = sscanf!(line, "{String}: {String} {String} {String}") {
            ret.insert(name, Monkey::Op(op.parse().unwrap(), Box::new(Monkey::Ref(left)), Box::new(Monkey::Ref(right))));
        }
    }
    ret
}

fn main() {
    let mut monkeys = read_input("inputs/21.txt");

    'outer: loop {
        let mut new_monkeys: HashMap<String,Monkey> = HashMap::new();
        for (name, mk) in &monkeys {
            match mk {
                Monkey::Literal(_) => {},
                Monkey::Ref(_) => unreachable!(),
                Monkey::Op(op, a, b) => {
                    let mut a = a.clone();
                    let mut b = b.clone();
                    if a.is_ref() && monkeys[a.as_monkey_ref().unwrap()].is_literal() {
                        a = Box::new(
                            Monkey::Literal(*monkeys[a.as_monkey_ref().unwrap()].as_literal().unwrap()));
                    }
                    if b.is_ref() && monkeys[b.as_monkey_ref().unwrap()].is_literal() {
                        b = Box::new(
                            Monkey::Literal(*monkeys[b.as_monkey_ref().unwrap()].as_literal().unwrap()));
                    }
                    if a.is_literal() && b.is_literal() {
                        let a = a.as_literal().unwrap();
                        let b = b.as_literal().unwrap();
                        if name == "root" {
                            println!("{}={}", a,b);
                            break 'outer;
                        }
                        let result = match op {
                            Op::Add => a + b,
                            Op::Div => a / b,
                            Op::Mul => a * b,
                            Op::Sub => a - b,
                        };
                        new_monkeys.insert(name.to_string(), Monkey::Literal(result));
                    } else {
                        new_monkeys.insert(name.to_string(), Monkey::Op(*op, a, b));
                    }
                },
            }
        }
        monkeys.extend(new_monkeys);
        if monkeys["root"].is_literal() {
            println!("Root screams {:?}", monkeys["root"]);
            break
        }
    }
}
