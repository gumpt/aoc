#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::str;

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn new(line: String) -> Passport {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(hgt|pid|eyr|cid|ecl|hcl|iyr|byr):(#?\w+)").unwrap();
        }

        let mut fields = HashMap::new();
        for captures in RE.captures_iter(&line) {
            let (field, value) = (captures[1].to_string(), captures[2].to_string());
            fields.insert(field, value);
        }

        Passport { fields }
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
                set.insert("cid".to_owned());
                set
            };
        }

        for key in REQUIRED.iter() {
            match self.fields.get(key) {
                Some(value) => {
                    match key.as_str() {
                        "byr" => {
                            // Birthyear as string should be four digits, >=1920 <=2002
                            if value.len() != 4 {
                                return false;
                            }

                            let parsed: usize = value.parse().unwrap();
                            if parsed < 1920 || parsed > 2002 {
                                return false;
                            }

                            continue;
                        }
                        "iyr" => {
                            // four digits; at least 2010 and at most 2020.
                            if value.len() != 4 {
                                return false;
                            }

                            let parsed: usize = value.parse().unwrap();
                            if parsed < 2010 || parsed > 2020 {
                                return false;
                            }

                            continue;
                        }
                        "eyr" => {
                            // four digits; at least 2020 and at most 2030.
                            if value.len() != 4 {
                                return false;
                            }

                            let parsed: usize = value.parse().unwrap();
                            if parsed < 2020 || parsed > 2030 {
                                return false;
                            }

                            continue;
                        }
                        "hgt" => {
                            lazy_static! {
                                // a number followed by either cm or in:
                                static ref HGT_RE: Regex = Regex::new(r"(\d+)((in)|(cm))$").unwrap();
                            }
                            match HGT_RE.captures(value) {
                                Some(inner) => {
                                    match &inner[2] {
                                        "cm" => {
                                            // If cm, the number must be at least 150 and at most 193.
                                            let parsed: usize = inner[1].parse().unwrap();
                                            if parsed < 150 || parsed > 193 {
                                                return false;
                                            }
                                            continue;
                                        }
                                        "in" => {
                                            // If in, the number must be at least 59 and at most 76.}
                                            let parsed: usize = inner[1].parse().unwrap();
                                            if parsed < 59 || parsed > 76 {
                                                return false;
                                            }
                                            continue;
                                        }
                                        _ => return false,
                                    }
                                }
                                None => return false,
                            }
                        }
                        "hcl" => {
                            // a # followed by exactly six characters 0-9 or a-f.
                            lazy_static! {
                                static ref HCL_RE: Regex = Regex::new(r"#[a-f0-9]{6}").unwrap();
                            }

                            if !HCL_RE.is_match(value) {
                                return false;
                            }
                            continue;
                        }
                        "ecl" => {
                            //  exactly one of: amb blu brn gry grn hzl oth
                            match value.as_str() {
                                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => continue,
                                _ => return false,
                            }
                        }
                        "pid" => {
                            // a nine-digit number, including leading zeroes.
                            if value.len() != 9 {
                                return false;
                            }
                            if let Err(_) = value.parse::<usize>() {
                                return false;
                            }
                            continue;
                        }
                        "cid" => continue,
                        // Extra key
                        _ => {
                            println!("{:?}", value);
                            return false;
                        }
                    }
                }
                None => {
                    if key == "cid" {
                        continue;
                    }
                    return false;
                }
            }
        }

        true
    }

    // fn add_field(mut self, field: &str) {
    //     self.fields.insert(field.to_string());
    // }
}

fn part_two() {
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
                println!("{:?}", line);
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

    // TODO Process what's left in the buffer
    let line = Passport::new(String::from_utf8(buf).unwrap());
    if line.is_valid() {
        println!("{:?}", line);
        count += 1;
    }

    println!("PART 2: {}", count);
}

fn main() {
    part_two();
}
