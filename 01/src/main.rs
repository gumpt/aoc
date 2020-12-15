use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let target = 2020;
    let mut set = HashSet::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let i: u64 = line.unwrap().parse().unwrap();
        set.insert(i);

        let remainder = target - i;
        if set.contains(&remainder) {
            println!("PART 1: {}", remainder * i);
        }
    }

    let iter = set.into_iter();
    for item in iter.combinations(3) {
        let (a, b, c) = (item[0], item[1], item[2]);
        if (a + b + c) == target {
            println!("PART 2: {}", a * b * c);
        }
    }
}
