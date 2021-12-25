#![feature(drain_filter)]
use std::io::{self, BufRead};

#[derive(Debug)]
struct Board {
    values: Vec<Vec<i64>>,
}

impl Board {
    fn new(lines: &[String]) -> Board {
        let values = lines.iter().fold(Vec::new(), |mut acc, v| {
            acc.push(
                v.split(' ')
                    .collect::<Vec<_>>()
                    .iter()
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse().unwrap())
                    .collect(),
            );
            acc
        });
        Board { values: values }
    }

    fn call_number(&mut self, num: &i64) -> Option<i64> {
        for i in 0..5 {
            for j in 0..5 {
                if self.values[i][j] == *num {
                    self.values[i][j] = -1;
                    if self.completed(i, j) {
                        return Some(self.score() * num);
                    }
                    return None;
                }
            }
        }
        return None;
    }

    fn completed(&self, i: usize, j: usize) -> bool {
        return self.values[i].iter().all(|x| x < &0)
            || self.values.iter().map(|row| row[j]).all(|x| x < 0);
    }

    fn score(&self) -> i64 {
        return self.values.iter().flatten().filter(|&&x| x > 1).sum();
    }
}

fn main() {
    let mut lines = Vec::new();
    let mut numbers: Vec<i64> = Vec::new();
    let mut boards = Vec::new();

    let stdin = io::stdin();
    for unwrapped in stdin.lock().lines() {
        let line = unwrapped.unwrap();
        if line.is_empty() {
            continue;
        }

        if numbers.is_empty() {
            numbers = line.split(',').map(|v| v.parse().unwrap()).collect();
            continue;
        }

        lines.push(line);
        if lines.len() == 5 {
            let b = Board::new(lines.as_slice());
            boards.push(b);
            lines.clear();
        }
    }

    let mut winners = Vec::new();
    for n in numbers.iter() {
        boards.drain_filter(|mut board| {
            if let Some(score) = board.call_number(n) {
                if winners.is_empty() {
                    println!("Part 1: {}", score);
                }
                winners.push(score);
                return true;
            }
            false
        });
    }
    winners.reverse();
    println!("Part 2: {}", winners[0]);
}
