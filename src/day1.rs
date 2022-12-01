use lib::*;

fn main() {
    let mut elves: Vec<u32> = read_lines("./inputs/1.txt")
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|x| x.iter().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    elves.sort_by(|a, b| b.cmp(a));
    println!("Best elf: {}", elves[0]);
    println!("Best three: {}", elves[0..3].into_iter().sum::<u32>());
}
