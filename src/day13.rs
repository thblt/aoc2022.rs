use lib::*;
use std::cmp::Ordering;
use std::fmt::Debug;

type Int = i64;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Token {
    Open,
    Close,
    Integer(Int),
}

// * Tokenizer

struct Tokenizer {
    tokens: Vec<Token>,
    number: Option<Int>,
}

impl Tokenizer {
    fn new() -> Tokenizer {
        Tokenizer {
            tokens: vec![],
            number: None,
        }
    }

    fn tokenize(s: &str) -> Vec<Token> {
        let mut state = Tokenizer::new();
        for c in s.chars() {
            match c {
                '[' => state.open(),
                ']' => state.close(),
                ',' => state.delim(),
                c if c.is_ascii_digit() => state.read_digit(read_digit(c)),
                _ => panic!("Invalid input."),
            }
        }
        state.delim(); // If the input was a single number, push it.
        state.tokens
    }

    fn finish_number(&mut self) {
        if let Some(n) = self.number {
            self.tokens.push(Token::Integer(n));
            self.number = None;
        }
    }

    fn push_non_digit(&mut self, t: Token) {
        self.finish_number();
        self.tokens.push(t);
    }

    fn open(&mut self) {
        self.push_non_digit(Token::Open);
    }

    fn close(&mut self) {
        self.push_non_digit(Token::Close);
    }

    fn delim(&mut self) {
        self.finish_number();
    }

    fn read_digit(&mut self, digit: u8) {
        self.number = Some(self.number.map_or(digit as Int, |n| n * 10 + digit as Int));
    }
}

// * Parser

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
enum Atom {
    Integer(Int),
    List(Vec<Atom>),
}

impl Atom {
    fn as_list(&self) -> Option<&Vec<Atom>> {
        if let Self::List(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_integer(&self) -> Option<&Int> {
        if let Self::Integer(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the atom is [`Integer`].
    ///
    /// [`Integer`]: Atom::Integer
    #[must_use]
    fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(..))
    }

    /// Returns `true` if the atom is [`List`].
    ///
    /// [`List`]: Atom::List
    #[must_use]
    fn is_list(&self) -> bool {
        matches!(self, Self::List(..))
    }
}

impl Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Integer(n) => n.fmt(f),
            Atom::List(l) => l.fmt(f),
        }
    }
}

struct Parser {
    // result: Atom,
    stack: Vec<Atom>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            stack: vec![Atom::List(vec![])],
        }
    }

    fn open(&mut self) {
        self.stack.push(Atom::List(vec![]))
    }

    fn close(&mut self) {
        let list = self.stack.pop().unwrap();
        if let Some(Atom::List(l)) = self.stack.last_mut() {
            l.push(list);
        } else {
            panic!("Abnormal input (unexpected end of list)");
        }
    }

    fn integer(&mut self, int: Int) {
        let parent = self.stack.last_mut().unwrap();
        if let Atom::List(l) = parent {
            l.push(Atom::Integer(int));
        } else {
            panic!("Abnormal state (no list to store that integer)");
        }
    }

    fn parse(s: &str) -> Atom {
        let tokens = Tokenizer::tokenize(s);
        Self::parse_tokens(&tokens)
    }

    fn parse_tokens(tokens: &[Token]) -> Atom {
        use Token::*;

        let mut state: Parser = Parser::new();
        for t in tokens {
            match *t {
                Open => state.open(),
                Close => state.close(),
                Token::Integer(n) => state.integer(n),
            }
        }
        // @FIXME: Add sanity check.
        // The following input parses: "17,3"
        // but isn't representable, and should fail.
        // Either return before end of token list (on state.clese()) or crash politely.
        state
            .stack
            .last()
            .unwrap()
            .as_list()
            .unwrap()
            .last()
            .unwrap()
            .clone()
    }
}

// * Problem

fn compare(left: &Atom, right: &Atom) -> Ordering {
    // Case 1: two integers.
    // println!("{} - Compare: {:?} with {:?}", " ".repeat(depth*2), left, right);
    use Ordering::*;
    if left.is_integer() && right.is_integer() {
        return left.as_integer().cmp(&right.as_integer())
        // Case 2: two lists
    } else if left.is_list() && right.is_list() {
        let mut left = left.as_list().unwrap().iter();
        let mut right = right.as_list().unwrap().iter();
        loop {
            let l = left.next();
            let r = right.next();
            if l.is_none() && r.is_some() {
                return Less;
            } else if l.is_some() && r.is_none() {
                return Greater;
            } else if l.is_none() && r.is_none() {
                break;
            } else {
                let ret = compare(l.unwrap(), r.unwrap());
                if ret != Equal {
                    return ret;
                }
            }
        }
        // Case 3 and 3’: list and integer.
    } else if left.is_integer() {
        let ret = compare(&Atom::List(vec![left.clone()]), right);
        if ret != Equal {
            return ret;
        }
    } else if right.is_integer() {
        let ret = compare(left, &Atom::List(vec![right.clone()]));
        if ret != Equal {
            return ret;
        }
    } else {
        unreachable!();
    }
    Equal
}

fn main() {
    let mut input = read_lines("inputs/13.txt");
    let mut index = 0;
    let mut sum = 0;

    let dp1 = Parser::parse("[[2]]");
    let dp2 = Parser::parse("[[6]]");

    let mut all: Vec<Atom> = vec![dp1.clone(),dp2.clone()];
    loop {
        let left = Parser::parse(&input.next().unwrap().unwrap());
        let right = Parser::parse(&input.next().unwrap().unwrap());

        // Collect for part 2
        all.push(left.clone());
        all.push(right.clone());

        index += 1;
         if compare(&left, &right) == Ordering::Less {
             sum += index;
        }
        if input.next().is_none() {
            break;
        }
    }
    println!("Part 1: {}", sum);

    all.sort_by(compare);
    let mut product = 1;
    for (i, item) in all.iter().enumerate() {
        if *item == dp1 || *item == dp2 {
            product *= i+1;
        }
    }
    println!("Decoder key: {}", product);
}
