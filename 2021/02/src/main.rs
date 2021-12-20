use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let mut window: VecDeque<i64> = VecDeque::new();

    let mut count: u64 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let i: i64 = line.unwrap().parse().unwrap();
        if window.len() < 3 {
            window.push_back(i);
            continue;
        }

        window.push_back(i);
        // TODO: track sums as we go :P
        let first: i64 = window.iter().take(3).sum();
        let second: i64 = window.iter().rev().take(3).sum();

        if second > first {
            count += 1;
        }
        println!("{:?}: {} {}", window, first, second);
        window.pop_front();
    }

    println!("PART 2: {}", count);
}
