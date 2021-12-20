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

fn get_addresses(mask: &HashMap<usize, usize>, location: usize) -> Vec<usize> {
    // Number of total expected addresses.
    let total = 2u64.pow((36 - mask.len()) as u32);

    let mut bases: Vec<usize> = vec![];
    for address in 0..total {
        let mut index = 0;
        let mut value = 0;

        for i in (0..36).rev() {
            value = value << 1;

            if mask.contains_key(&i) {
                match mask.get(&i).unwrap() {
                    0 => {
                        // Enforce the bit is set to 0.
                        let bit = (location >> i) & 1;
                        if bit == 1 {
                            value |= 1
                        };
                    }
                    1 => value |= 1,
                    _ => panic!("ack"),
                }
                continue;
            }

            // Otherwise, toggle this floating bit based on how far we are into
            // our loop of the total (ie the low bit of 0..total).
            let bit = (address >> index) & 1;
            index += 1;
            if bit == 1 {
                value |= 1;
            }
        }

        bases.push(value);
    }

    bases
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
            for address in get_addresses(&mask, location) {
                mem.insert(address, value);
            }
        }
    }

    println!("PART 2: {}", mem.values().sum::<usize>());
}
