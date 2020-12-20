use itertools::Itertools;
use std::io::BufRead;

fn search(elements: Vec<usize>) -> usize {
    let copy = elements.clone();

    let mut splits = vec![];
    for (i, window) in copy.windows(2).enumerate() {
        let (first, second) = (window[0], window[1]);
        if second - first == 3 {
            splits.push(i);
        }
    }

    splits.push(0);
    splits.sort();

    let mut partitions = vec![];
    let copy = elements.clone();
    for split in splits.windows(2) {
        let (mut first, second) = (split[0], split[1]);
        // indices are hard!
        if first != 0 {
            first += 1;
        }

        let partition = copy[first..=second].to_vec();
        partitions.push(partition);
    }

    partitions.push(vec![*elements.last().unwrap()]);

    let r = partitions.iter().fold(1, |acc, partition| {
        // println!("{:?}", partition);
        // partitions.iter().for_each(|partition| {
        let mut body = partition.clone();
        let last = body.pop().unwrap();
        if body.is_empty() {
            return acc;
        }

        let first = body.remove(0);
        if body.is_empty() {
            return acc;
        }
        let mut running = 0;
        for len in 0..=body.len() {
            let permutations = body
                .clone()
                .into_iter()
                .permutations(len)
                .filter(|p| is_valid_set(p, first, last))
                .collect_vec();

            running += permutations.len();
        }

        acc * running
    });

    return r;
}

fn is_valid_set(permutation: &Vec<usize>, first: usize, last: usize) -> bool {
    let mut list = vec![first];
    list.extend(permutation.clone());
    list.push(last);

    for window in list.windows(2) {
        let (first, second) = (window[0] as i64, window[1] as i64);
        match second - first.abs() {
            0..=3 => continue,
            _ => return false,
        }
    }

    true
}

fn main() {
    let mut elements = Vec::new();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let parsed: usize = line.unwrap().parse().unwrap();
        elements.push(parsed);
    }

    elements.push(0);
    elements.sort();
    elements.push(elements.last().unwrap() + 3);

    let mut count_one = 0;
    let mut count_three = 0;
    for window in elements.windows(2) {
        let (first, second) = (window[0], window[1]);
        match second - first {
            1 => count_one += 1,
            3 => count_three += 1,
            0 | 2 => continue,
            _ => panic!("ack!"),
        }
    }

    println!("PART ONE: {}", count_three * count_one);

    println!("PART TWO: {}", search(elements));
}
