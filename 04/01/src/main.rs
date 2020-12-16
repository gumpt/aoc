#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str;

struct Passport {
    fields: HashSet<String>,
}

impl Passport {
    fn new(line: String) -> Passport {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(hgt|pid|eyr|cid|ecl|hcl|iyr|byr):(#?\w+)").unwrap();
        }

        let mut set = HashSet::new();
        for captures in RE.captures_iter(&line) {
            let (field, _value) = (captures[1].to_string(), captures[2].to_string());
            set.insert(field);
        }

        Passport { fields: set }
    }

    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref REQUIRED: HashSet<String> = {
                let mut set = HashSet::new();
                set.insert("byr".to_owned());
                set.insert("iyr".to_owned());
                set.insert("eyr".to_owned());
                set.insert("hgt".to_owned());
                set.insert("hcl".to_owned());
                set.insert("ecl".to_owned());
                set.insert("pid".to_owned());
                set
            };
        }

        REQUIRED.is_subset(&self.fields)
    }

    // fn add_field(mut self, field: &str) {
    //     self.fields.insert(field.to_string());
    // }
}

fn part_one() {
    let stdin = io::stdin();

    lazy_static! {
        static ref REQUIRED_FIELDS: HashSet<&'static str> =
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
                .iter()
                .cloned()
                .collect();
    }

    let mut count = 0;
    let mut buf = vec![];
    for line in stdin.lock().lines() {
        let mut unwrapped = line.unwrap();
        if unwrapped == "" {
            // Process what's in the buffer
            let line = Passport::new(String::from_utf8(buf).unwrap());
            if line.is_valid() {
                count += 1;
            }

            buf = vec![];
            continue;
        }

        if buf.len() != 0 {
            unwrapped = " ".to_owned() + &unwrapped;
        }

        buf.append(&mut unwrapped.as_bytes().to_vec());
    }

    // Process what's left in the buffer
    let line = Passport::new(String::from_utf8(buf).unwrap());
    if line.is_valid() {
        count += 1;
    }

    println!("PART 1: {}", count);
}

fn main() {
    part_one();
}
