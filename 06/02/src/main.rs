#![feature(iterator_fold_self)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result: usize = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| {
                    person
                        .chars()
                        .filter(|&c| !c.is_whitespace())
                        .collect::<HashSet<_>>()
                })
                .fold_first(|a, b| &a & &b)
                .unwrap()
                .len()
        })
        .sum();

    println!("Part 2: {}", result);
}
