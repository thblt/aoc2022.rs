use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

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
