use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result: usize = input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| !c.is_whitespace())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();

    println!("Part 1: {}", result);
}
