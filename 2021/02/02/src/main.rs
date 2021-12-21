use std::io::{self, BufRead};

#[derive(Debug)]
struct Submarine {
    depth: i64,
    horizontal: i64,
    aim: i64,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }

    fn forward(&mut self, distance: i64) {
        self.horizontal += distance;
        self.depth += distance * self.aim;
    }

    fn up(&mut self, distance: i64) {
        self.aim -= distance;
    }

    fn down(&mut self, distance: i64) {
        self.aim += distance;
    }
}

fn main() {
    let mut sub = Submarine::new();

    let stdin = io::stdin();
    for unwrapped in stdin.lock().lines() {
        let line = unwrapped.unwrap();
        let splat: Vec<&str> = line.split(' ').take(2).collect();

        let (action, distance) = (splat[0], splat[1].parse::<i64>().unwrap());
        match action {
            "up" => sub.up(distance),
            "down" => sub.down(distance),
            "forward" => sub.forward(distance),
            _ => println!("NUPE"),
        }
    }

    println!("{:?}: {}", sub, sub.depth * sub.horizontal);
    // println!("PART 1: {}", count);
}
