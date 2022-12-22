use lib::*;
use sscanf::sscanf;

type Coord3 = (isize,isize,isize);

fn manhattan3 (a: &Coord3, b: &Coord3) -> isize {
    abs_diff(a.0, b.0) +
        abs_diff(a.1, b.1) +
        abs_diff(a.2, b.2)
}

fn read_input(s: &str) -> Vec<Coord3> {
    let mut ret = vec!();
    for line in read_lines(s) {
        let line = line.unwrap();
        ret.push(sscanf!(line, "{isize},{isize},{isize}").unwrap());
    }
    ret
}

fn main(){
    let input = read_input("inputs/18.txt");

    let mut count = 0;
    for a in 0..input.len() {
        for b in a..input.len() {
            if manhattan3(&input[a], &input[b]) == 1 {
                count += 1;
            }
        }
    }
    println!("{}", input.len()*6 - count*2);

}
