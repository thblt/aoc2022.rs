use lib::*;

fn read_input(s: &str) -> Vec<i32> {
    read_lines(s).map(|l| l.unwrap().parse::<i32>().unwrap()).collect()
}

fn find_nth(vec: &Vec<(usize,i32)>, idx: usize) -> Option<usize> {
    vec.iter().enumerate().find_map(|(i, (x, _))| if *x == idx { Some (i) } else {None} )
}
/// Indices start at 1
fn wrapping_nth<T>(vec: &Vec<T>, idx: usize) -> &T {
    let idx = idx % vec.len();
    &vec[idx]
}


fn main() {

    let expect = vec![
        "1, 2, -3, 3, -2, 0, 4",
        "2, 1, -3, 3, -2, 0, 4",
        "1, -3, 2, 3, -2, 0, 4",
        "1, 2, 3, -2, -3, 0, 4",
        "1, 2, -2, -3, 0, 3, 4",
        "1, 2, -3, 0, 3, 4, -2",
        "1, 2, -3, 0, 3, 4, -2",
        "1, 2, -3, 4, 0, 3, -2",
    ];

    let mut input: Vec<(usize, i32)> = read_input("inputs/20
.txt").into_iter().enumerate().collect();
    let count = input.len();
    println!("{count} elements.");

    for i in 0..count {
        // print!("Our state: ");
        // for (_, item) in &input {
        //     print!("{}, ", item);
        // }
        // println!();
        // println!(" Expected: {},\n", expect[i]);

        // Remove item
        let idx = find_nth(&input, i).unwrap();
        let moved_item = input[idx];
        input = input.iter().filter(|(n, _)| i != *n).copied().collect();

        // Insert it back
        let mut mv = idx as i32 + moved_item.1;
        if mv == 0 {
            mv = count as i32;
        }
        // print!(" - Item {} (originally at {i}, now at {idx}) goes to {mv}, its next index is…", moved_item.1);

        if mv < 0 {
            // println!("NegSolver 2000 step 1: {}", mv);
            mv -= 1;
            while mv < 0 {
                mv = count as i32 + mv;
                // println!("NegSolver 2000 interm: {}", mv);
            }
            // println!("NegSolver 2000: final {}", mv);
        }
        if mv > count as i32 {
            mv %= count as i32 - 1;
        }
        // println!("{mv}");
        let mut input2 = vec!();
        let mut inserted = false;
        for (i, item) in input.drain(0..).enumerate() {
            if i == mv as usize {
                input2.push(moved_item);
                inserted = true;
            }
            input2.push(item);
        }
        if !inserted {
            input2.push(moved_item);
        }
        input = input2;

    }
    print!("Final state: ");
    for (_, item) in &input {
        print!("{}, ", item);
    }

    println!("\n   Expected: {},\n", expect.last().unwrap());

    let mut zero = 0;
    for i in 0..input.len() {
        if input[i].1 == 0 {
            zero = i;
            break;
        }
    }

    let a = wrapping_nth(&input, 1000+zero).1;
    let b = wrapping_nth(&input, 2000+zero).1;
    let c = wrapping_nth(&input, 3000+zero).1;

    println!("{a:?}+{b:?}+{c:?}={}", a+b+c);
}

// A B C D E F
