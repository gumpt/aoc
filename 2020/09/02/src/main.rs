use std::io::BufRead;
use std::vec::Vec;

fn main() {
    let mut values = Vec::new();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let value: usize = line.unwrap().parse().unwrap();
        values.push(value);
    }

    // Run part one to get/set this value.
    let target: usize = 776203571;

    let (mut i, mut j) = (0, 0);
    let mut slice = &values[i..=j];
    let mut sum: usize = slice.iter().sum();
    while sum != target {
        while sum < target {
            j += 1;
            slice = &values[i..=j];
            sum = slice.iter().sum();
        }

        while sum > target {
            i += 1;
            slice = &values[i..=j];
            sum = slice.iter().sum();
        }
    }

    println!(
        "PART TWO: {:?}",
        slice.iter().min().unwrap() + slice.iter().max().unwrap()
    );
}
