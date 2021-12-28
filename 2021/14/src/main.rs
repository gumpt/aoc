use itertools::Itertools;
use std::collections::HashMap;

type Pair = (char, char);

#[derive(Debug)]
struct Polymer {
    final_char: char,
    pairs: HashMap<Pair, usize>,
    rules: HashMap<Pair, Vec<Pair>>,
}

impl Polymer {
    fn new(template: &str) -> Polymer {
        let mut pair_counts: HashMap<Pair, usize> = HashMap::new();
        for pair in template.chars().tuple_windows() {
            let counter = pair_counts.entry(pair).or_insert(0);
            *counter += 1;
        }

        Polymer {
            final_char: template.chars().last().unwrap(),
            pairs: pair_counts,
            rules: HashMap::new(),
        }
    }

    fn add_rule(&mut self, rule_str: &str) {
        let mut both = rule_str.split("->");
        let (rule, insertion): (Vec<char>, char) = (
            both.next().unwrap().trim().chars().collect(),
            both.next().unwrap().trim().chars().nth(0).unwrap(),
        );
        let r = Rule::new(rule.as_slice(), insertion);
        let mut outputs = self.rules.entry(r.start).or_insert(Vec::new());
        outputs.extend(r.output);
    }

    fn run_insertion(&mut self) {
        let mut new_counts = HashMap::new();
        for couple in self.pairs.iter() {
            let (pair, n): (&Pair, &usize) = couple;
            let rule = self.rules.get(pair).unwrap();
            for r in rule {
                *new_counts.entry(*r).or_default() += *n
            }
        }
        self.pairs = new_counts;
    }

    fn most_common(&self) -> usize {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for (key, value) in &self.pairs {
            let first = counts.entry(key.0).or_default();
            *first += value;
        }
        *counts.entry(self.final_char).or_default() += 1;
        println!("MAX {:?}", counts);
        *counts.values().max().unwrap()
    }

    fn least_common(&self) -> usize {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for (key, value) in &self.pairs {
            let first = counts.entry(key.0).or_insert(0);
            *first += value;
        }
        *counts.entry(self.final_char).or_default() += 1;
        *counts.values().min().unwrap()
    }
}

#[derive(Debug)]
struct Rule {
    start: Pair,
    output: Vec<Pair>,
}

impl Rule {
    fn new(start: &[char], insert: char) -> Rule {
        let pair = (start[0], start[1]);
        let pairs = vec![(start[0], insert), (insert, start[1])];

        Rule {
            start: pair,
            output: pairs,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut split = input.split("\n\n");

    let template = split.next().unwrap();
    let mut polymer = Polymer::new(template);
    for line in split.next().unwrap().lines() {
        polymer.add_rule(line);
    }

    for i in 0..40 {
        polymer.run_insertion();
        println!("{}", i);
        println!("{:?}", polymer.pairs);
        println!("{}", i);
    }

    println!(
        "Part 1: {} {} {}",
        polymer.most_common(),
        polymer.least_common(),
        polymer.most_common() - polymer.least_common()
    );
}
