use std::collections::HashSet;
use std::io::BufRead;
use std::vec::Vec;

fn generate_sums(buf: &Vec<usize>) -> HashSet<usize> {
    let mut sums = HashSet::new();

    for i in 0..buf.len() {
        for j in 0..buf.len() {
            if i == j {
                continue;
            }

            sums.insert(buf[i] + buf[j]);
        }
    }

    return sums;
}

fn main() {
    let stdin = std::io::stdin();

    let mut buf = vec![0; 25];

    for (i, line) in stdin.lock().lines().enumerate() {
        let value: usize = line.unwrap().parse().unwrap();

        // Preamble
        if i < 25 {
            buf[i % 25] = value;
            continue;
        }

        let sums = generate_sums(&buf);
        if !sums.contains(&value) {
            println!("PART 1: {}", value);
            return;
        }

        buf[i % 25] = value;
    }
}
