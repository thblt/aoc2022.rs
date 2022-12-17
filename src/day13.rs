use lib::*;
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
                c if c.is_digit(10) => state.read_digit(read_digit(c)),
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
    List(Vec<Box<Atom>>),
}

impl Atom {
    fn as_list(&self) -> Option<&Vec<Box<Atom>>> {
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
            l.push(Box::new(list));
        } else {
            panic!("Abnormal input (unexpected end of list)");
        }
    }

    fn integer(&mut self, int: Int) {
        let parent = self.stack.last_mut().unwrap();
        if let Atom::List(l) = parent {
            l.push(Box::new(Atom::Integer(int)));
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
        *state
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

enum ComparisonResult {
    Valid,
    Invalid,
    Undecided,
}

fn compare(left: &Atom, right: &Atom, depth: usize) -> Option<bool> {
    // Case 1: two integers.
    // println!("{} - Compare: {:?} with {:?}", " ".repeat(depth*2), left, right);
    if left.is_integer() && right.is_integer() {
        if left.as_integer() < right.as_integer() {
            return Some(true);
        } else if left.as_integer() > right.as_integer() {
            return Some(false);
        } else {
            return None;
        };
        // Case 2: two lists
    } else if left.is_list() && right.is_list() {
        let mut left = left.as_list().unwrap().iter();
        let mut right = right.as_list().unwrap().iter();
        loop {
            let l = left.next();
            let r = right.next();
            if l.is_none() && r.is_some() {
                return Some(true);
            } else if l.is_some() && r.is_none() {
                return Some(false);
            } else if l.is_none() && r.is_none() {
                break;
            } else {
                let ret = compare(l.unwrap(), r.unwrap(),depth+1);
                if ret.is_some() { return ret; }

            }
        }
        // Case 3 and 3’: list and integer.
    } else if left.is_integer() {
        let ret = compare(&Atom::List(vec![Box::new(left.clone())]), right,depth+1);
        if ret.is_some() { return ret; }
    } else if right.is_integer() {
        let ret = compare(left, &Atom::List(vec![Box::new(right.clone())]),depth+1);
        if ret.is_some() { return ret; }
    } else {
        unreachable!();
    }
    None
}

fn main() {
    let mut input = read_lines("inputs/13.txt");
    let mut index = 0;
    let mut sum = 0;
    loop {
        let left = Parser::parse(&input.next().unwrap().unwrap());
        let right = Parser::parse(&input.next().unwrap().unwrap());
        index += 1;
        // println!("{:?}", left);
        // println!("{:?}", right);
        if compare(&left, &right, 0) == Some(true) {
            sum += index;
    }
    if input.next().is_none() {
        break;
    }
    }
    println!("Sum: {}", sum);
}
