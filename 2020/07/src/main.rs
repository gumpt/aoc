#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

type Subgraph = HashMap<String, usize>;
type Graph = HashMap<String, Subgraph>;

fn main_bag_name(part: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w+ \w+) bags").unwrap();
    }

    return RE
        .captures(part)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
}

fn bags_and_quantities(part: &str) -> Subgraph {
    lazy_static! {
        static ref RE: Regex = Regex::new(r" ?((\d+) (\w+ \w+) bags?[,.]?)+").unwrap();
    }

    let mut map = HashMap::new();
    RE.captures_iter(part).for_each(|part| {
        let count: usize = part.get(2).unwrap().as_str().parse().unwrap();
        let color = part.get(3).unwrap().as_str().to_owned();
        map.insert(color, count);
    });

    return map;
}

fn contains_shiny_gold(color: &str, graph: &Graph, memo: &mut HashMap<String, bool>) -> bool {
    if let Some(&v) = memo.get(color) {
        return v;
    }

    let indirect = graph
        .get(color)
        .unwrap()
        .iter()
        .fold(false, |acc, (c, _count)| {
            acc | contains_shiny_gold(c, graph, memo)
        });

    memo.insert(color.to_owned(), indirect);
    return indirect;
}

fn inner_size(color: &str, graph: &Graph, memo: &mut Subgraph) -> usize {
    if let Some(&v) = memo.get(color) {
        return v;
    }

    let size = graph.get(color).unwrap().iter().fold(0, |acc, (c, count)| {
        acc + count + (count * inner_size(c, graph, memo))
    });

    memo.insert(color.to_owned(), size);
    return size;
}

fn main() {
    let stdin = io::stdin();

    let mut graph: Graph = HashMap::new();
    for line in stdin.lock().lines() {
        let unwrapped = line.unwrap();
        let parts: Vec<&str> = unwrapped.split("contain").collect();

        let color = main_bag_name(parts[0]);
        let subgraph = bags_and_quantities(parts[1]);
        graph.insert(color, subgraph);
    }

    let mut memo: HashMap<String, bool> = HashMap::new();
    let query = "shiny gold";
    // Insert all directly containing gold to start.
    for (root, subgraph) in graph.iter() {
        if subgraph.contains_key(query) {
            memo.insert(root.to_owned(), true);
        }

        if subgraph.is_empty() {
            memo.insert(root.to_owned(), false);
        }
    }

    let count = graph.keys().fold(0, |acc, color| {
        match contains_shiny_gold(&color, &graph, &mut memo) {
            true => acc + 1,
            false => acc,
        }
    });

    println!("PART ONE: {}", count);

    let mut inner_memo: Subgraph = HashMap::new();
    // Insert all 0 sized to start
    for (root, subgraph) in graph.iter() {
        if subgraph.is_empty() {
            inner_memo.insert(root.to_owned(), 0);
        }
    }
    println!(
        "PART TWO: {}",
        inner_size("shiny gold", &graph, &mut inner_memo)
    );
}
