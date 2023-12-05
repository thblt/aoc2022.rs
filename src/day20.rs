use lib::*;

fn read_input(s: &str) -> Vec<i64> {
    read_lines(s).map(|l| l.unwrap().parse::<i64>().unwrap()).collect()
}

fn decrypt(input: &mut Vec<(usize,i64)>, repeat: usize) {
    let count = input.len() as isize;

    for _ in 0..repeat {
        for idx in 0..count as usize {
            let mut index = input.iter().position(|(i,_)| *i==idx).unwrap();
            let motion = input[index].1 % (count - 1) as i64;
            let mut swap_with: isize;
            for step in std::iter::repeat(if motion < 0 { -1 } else { 1 }).take(motion.abs() as usize) {
                swap_with = index as isize + step;
                while swap_with < 0 {
                    swap_with += count;
                }
                swap_with %= count;
                input.swap(index, swap_with as usize);
                index = swap_with as usize
            }
        }
    }

    let zero = input.iter().position(|(_,v)| *v==0).unwrap();
    let mut result = 0;
    for i in 1..4 {
        result += input[(zero + i*1000) % input.len()].1;
    }
    println!("Grove location: {result}");
}

fn main() {
    let mut input1: Vec<(usize, i64)> = read_input("inputs/20.txt").into_iter().enumerate().collect();
    let mut input2: Vec<(usize, i64)> = read_input("inputs/20.txt").into_iter().map(|x| x*811589153).enumerate().collect();
    decrypt(&mut input1, 1);
    decrypt(&mut input2, 10);
}

// A B C D E F
