#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

fn parse_mask(line: &str) -> HashMap<usize, usize> {
    let (_, mask_str) = line.split_at(7);

    let mut mask = HashMap::new();
    for (i, c) in mask_str.chars().enumerate() {
        match c {
            '1' => mask.insert(35 - i, 1),
            '0' => mask.insert(35 - i, 0),
            _ => continue,
        };
    }

    mask
}

fn parse_memset(line: &str) -> (usize, usize) {
    lazy_static! {
        static ref MEM_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }
    let captures = MEM_RE.captures(line).unwrap();
    let (location, value) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());

    (location, value)
}

fn apply_mask(mask: &HashMap<usize, usize>, value: usize) -> usize {
    let mut masked = value;
    for (i, v) in mask.iter() {
        match v {
            1 => masked |= 1 << i,
            0 => masked &= !(1 << i),
            _ => continue,
        };
    }

    return masked;
}

fn main() {
    let stdin = std::io::stdin();

    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask = HashMap::new();

    for line in stdin.lock().lines() {
        let text = line.unwrap();
        if text.starts_with("mask") {
            mask = parse_mask(&text);
        } else if text.starts_with("mem") {
            let (location, value) = parse_memset(&text);
            let masked = apply_mask(&mask, value);
            mem.insert(location, masked);
        }
    }

    println!("PART 1: {}", mem.values().sum::<usize>());
}
