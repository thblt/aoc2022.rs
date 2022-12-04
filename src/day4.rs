use lib::*;
use sscanf::sscanf;

type InputLine = (u32, u32,u32, u32);

fn read_input(path: &str) -> Vec<InputLine> {
    let mut ret: Vec<InputLine> = vec!();
    for line in read_lines(path) {
        let line = line.unwrap();
        ret.push(sscanf!(line, "{u32}-{u32},{u32}-{u32}").unwrap());
    }
    ret
}

fn main() {
    let mut count1 = 0;
    let mut count2 = 0;
    for (a1,a2,b1,b2) in read_input("inputs/4.txt") {
        if (a1 >= b1 && a2 <= b2) || (a1 <= b1 && a2 >= b2){
            count1 += 1;
        }
        if !((a1 < b1 && a2 < b1) || (a1 > b1 && a1 > b2)) {
            count2 += 1;
        }
    }
    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);
}
