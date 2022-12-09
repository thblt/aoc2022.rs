use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::ops::Sub;

pub mod matrix;

pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub fn bools_to_bin_string(n: &[bool]) -> String {
    n.iter().map(|x| if *x { '1' } else { '0' }).collect()
}

pub fn read_digit(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => panic!("Bad input"),
    }
}


pub fn abs_diff<T: Copy + Ord + Sub>(a: T, b: T) -> <T as Sub>::Output  {
    use std::cmp::{min,max};
    max(a, b) - min (a, b)
}
