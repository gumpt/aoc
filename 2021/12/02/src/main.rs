use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, name: &str) {
        if self.adjacency_list.contains_key(name) {
            return;
        }
        self.adjacency_list.insert(name.to_owned(), HashSet::new());
    }

    fn add_edge(&mut self, first: &str, second: &str) {
        self.adjacency_list
            .get_mut(first)
            .unwrap()
            .insert(second.to_owned());
        self.adjacency_list
            .get_mut(second)
            .unwrap()
            .insert(first.to_owned());
    }

    fn all_paths(
        &self,
        visiting: &str,
        end: &str,
        mut revisiting: bool,
        mut path: Vec<String>,
    ) -> HashSet<String> {
        if visiting == end {
            path.push(visiting.to_owned());
            return HashSet::from([path.join(",")]);
        }

        let mut paths = HashSet::new();
        if !visiting.chars().nth(0).unwrap().is_uppercase() {
            if path.contains(&visiting.to_owned()) {
                if revisiting || visiting == "start" {
                    return paths;
                }
                revisiting = true;
            }
        }

        path.push(visiting.to_owned());
        for v in self.adjacency_list.get(visiting).unwrap().iter() {
            paths.extend(self.all_paths(v, end, revisiting, path.clone()));
        }

        paths
    }
}

fn main() {
    let mut graph = Graph::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let unwrapped = line.unwrap();
        let parts: Vec<&str> = unwrapped.split('-').collect();
        graph.add_vertex(parts[0]);
        graph.add_vertex(parts[1]);
        graph.add_edge(parts[0], parts[1]);
    }

    let paths = graph.all_paths("start", "end", false, Vec::new());
    println!("{:?} {}", paths, paths.len());
}
