use std::io::{self, BufRead};

fn main() {
    let mut last: Option<i64> = None;

    let mut count: u64 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let i: i64 = line.unwrap().parse().unwrap();
        if last.is_none() {
            last = Some(i);
            continue;
        }

        let diff: i64 = i - last.unwrap();
        if diff > 0 {
            count += 1;
        }
        last = Some(i);
    }

    println!("PART 1: {}", count);
}
