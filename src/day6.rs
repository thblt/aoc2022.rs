use lib::*;

fn main() {
    let input: Vec<char> = read_lines("inputs/6.txt").next().unwrap().unwrap().chars().collect();

    let mut part1found = false;
    let mut range = (0,0);
    for idx in 0..input.len() {
        if let Some(dupl) = input[range.0..range.1]
            .iter()
            .enumerate()
            .find_map(|(n,item)| if item == &input[idx] { Some(n+range.0) } else { None }) {
                range.0 = dupl+1;
            }
        if range.1 - range.0 == 4 && !part1found {
            println!("Part 1: {} ({}-{} = {:?})", idx, range.0, range.1, &input[range.0..range.1]);
            part1found = true;
        }
        if range.1 - range.0 == 14 {
            println!("Part 2: {} ({}-{} = {:?})", idx, range.0, range.1, &input[range.0..range.1]);
            return
        } else {
            range.1 = idx+1;
        }
    }

}
