#[macro_use]
extern crate lazy_static;
use regex::{Captures, Regex};
use std::io::{self, BufRead};

#[derive(Debug)]
struct Line<'a> {
    low: usize,
    high: usize,
    c: &'a str,
    password: &'a str,
}

impl Line<'_> {
    fn parts(line: &str) -> Captures {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (.): (\w+)").unwrap();
        }
        return RE.captures(line).unwrap();
    }

    fn new<'a>(parts: &'a Captures) -> Line<'a> {
        let l = Line {
            low: parts[1].parse().unwrap(),
            high: parts[2].parse().unwrap(),
            c: &parts[3],
            password: &parts[4],
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
        let unwrapped = line.unwrap();
        let parts = Line::parts(&unwrapped);
        let l = Line::new(&parts);
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
