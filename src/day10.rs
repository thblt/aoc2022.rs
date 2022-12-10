use lib::*;
use sscanf::sscanf;

#[derive(Debug)]
struct Device {
    cycle: i64,
    x: i64,
    // crt: [bool; 40*6],

    counter: i64,
    next_event: i64
}

impl Device {
    fn new() -> Device {
        Device { cycle: 0,
                 x: 1,
                 // crt: [false;40*6],
                 counter: 0,
                 next_event: 20
        }
    }

    fn cycle(&mut self, incr: i64) {
        // Part 2
        let pixel = self.cycle % 40;
        if pixel >= self.x -1 && pixel <= self.x + 1 {
            print!("█");
        } else {
            print!(" ");
        }
        if (self.cycle + 1) % 40 == 0 {
            println!();
        }

        self.cycle += 1;
        // Part 1
        if self.cycle >= self.next_event {
            self.counter += self.cycle * self.x;
            self.next_event += 40;
        }
        self.x += incr;
    }
}

fn main() {
    let mut device = Device::new();
    for line in read_lines("inputs/10.txt") {
        let line =  line.unwrap();
        device.cycle(0);

        if let Ok(incr) = sscanf!(line, "addx {i64}") {
            device.cycle(incr);
        }
    }
}
