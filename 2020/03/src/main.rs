use std::io::{self, BufRead};

fn part_one() {
    let tree = '#' as u8;

    let stdin = io::stdin();
    let shift_right = 3;
    let mut right = 0;

    let mut count = 0;
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let bytes = l.as_bytes();
        if bytes[right % bytes.len()] == tree {
            count += 1;
        }

        right += shift_right;
    }

    println!("PART 1: {}", count);
}

fn part_two() {
    let tree = '#' as u8;

    let stdin = io::stdin();
    let shifts = [1, 3, 5, 7, 1];
    let mut rights = [0, 0, 0, 0, 0];
    let mut counts = [0, 0, 0, 0, 0];

    let mut count = 0;
    let mut rowsCounted = 0;

    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let bytes = l.as_bytes();
        for i in (0..shifts.len()) {
            // Only count the last shift every other row.
            if rowsCounted > 0 && rowsCounted % 2 != 0 && i == 4 {
                continue;
            }

            let right = rights[i];
            if bytes[right % bytes.len()] == tree {
                counts[i] += 1;
            }

            rights[i] += shifts[i];
        }

        rowsCounted += 1;
    }

    let mut product: usize = 1;
    for count in counts.iter() {
        product *= count;
    }

    println!("PART 2: {}", product);
}

fn main() {
    part_two()
}
