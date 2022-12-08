use lib::*;
use sscanf::sscanf;
use std::collections::HashMap;

fn main() {
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut stack: Vec<String> = vec![];
    let mut available = 70000000;
    for line in read_lines("inputs/7.txt") {
        let line = line.unwrap();
        if let Ok(cd) = sscanf!(&line, "$ cd {str}") {
            match cd {
                ".." => {
                    stack.pop();
                }
                "/" => { /* noop */ }
                _ => {
                    stack.push(cd.to_string());
                }
            }
            // } else if let Ok(dir) = sscanf!(&line, "dir {str}") {
            // stack.push(dir.to_string());
            // let repr = stack.join("/");
            // dirs.entry(repr).or_insert(0);
        } else if let Ok((size, _)) = sscanf!(&line, "{u32} {str}") {
            available -= size;
            for i in 0..=stack.len() {
                let key = stack[0..i].to_vec().join("/");
                let old_size = dirs.get(&key).unwrap_or(&0);
                dirs.insert(key, size + old_size);
            }
        }
    }

    let need = 30000000 - available;
    let mut total = 0;
    let mut best = u32::MAX;

    for (_, size) in dirs {
        if size <= 100000 {
            total += size;
        }
        if size >= need && size <= best {
            best = size;
        }
    }
    println!("Part 1: Total size of dirs < 100000 ......................... {:8}", total);
    println!("Part 2: Size of smallest dir that would free enough space ... {:8}", best);
}
