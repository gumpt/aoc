use regex::Regex;
use std::io::{self, BufRead};
use std::string::String;

#[derive(Debug)]
struct Line {
    low: usize,
    high: usize,
    c: String,
    password: String,
}

impl Line {
    fn new(line: &str) -> Line {
        let re = Regex::new(r"(\d+)-(\d+) (.): (\w+)").unwrap();
        let parts = re.captures(line).unwrap();

        // TODO: Don't use std::String use a str or something I dunno?
        // This runs real slow and I dunno if it's Rust strings or regexes!
        let l = Line {
            low: parts[1].parse().unwrap(),
            high: parts[2].parse().unwrap(),
            c: parts[3].to_string(),
            password: parts[4].to_string(),
        };
        // println!("{:?}", l);
        l
    }

    fn is_valid_one(&self) -> bool {
        let times = self.password.matches(&self.c).count();
        (self.low <= times) && (times <= self.high)
    }

    fn is_valid_two(&self) -> bool {
        let c = self.c.as_bytes()[0];
        let first = self.password.as_bytes()[self.low - 1] == c;
        let second = self.password.as_bytes()[self.high - 1] == c;
        first != second
    }
}

fn main() {
    let stdin = io::stdin();

    let mut count_one = 0;
    let mut count_two = 0;
    for line in stdin.lock().lines() {
        let l = Line::new(&line.unwrap());
        if l.is_valid_one() {
            count_one += 1;
        }

        if l.is_valid_two() {
            count_two += 1;
        }
    }

    println!("PART 1: {}", count_one);
    println!("PART 2: {}", count_two);
}
